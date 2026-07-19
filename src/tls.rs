//! In-app TLS via rustls with Let's Encrypt certificates issued over ACME.
//!
//! TLS mode is enabled entirely through environment variables:
//! - `ARS_TLS_DOMAIN` (required): domain to certify, e.g. api.ars.wiki
//! - `ARS_TLS_EMAIL`: contact email for the Let's Encrypt account
//! - `ARS_TLS_CERT_DIR`: cert/key/account storage (default `./certs`)
//! - `ARS_TLS_HTTP_PORT`: HTTP-01 challenge + redirect port (default 80)
//! - `ARS_TLS_STAGING=1`: use the Let's Encrypt staging directory
//! - `ARS_TLS_SELF_SIGNED=1`: skip ACME, generate a self-signed cert (local testing)

use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::extract::{Path as AxumPath, State};
use axum::http::{StatusCode, Uri};
use axum::response::Redirect;
use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use instant_acme::{
    Account, AccountCredentials, AuthorizationStatus, ChallengeType, Identifier, LetsEncrypt,
    NewAccount, NewOrder, Order, OrderStatus,
};
use tokio::time::sleep;
use tracing::{error, info};

const RENEW_AFTER_SECS: u64 = 60 * 24 * 60 * 60; // 60 days
const RENEW_CHECK_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);

type ChallengeStore = Arc<RwLock<HashMap<String, String>>>;

#[derive(Clone, Debug)]
pub struct TlsConfig {
    pub domain: String,
    pub email: Option<String>,
    pub cert_dir: PathBuf,
    pub http_port: u16,
    pub staging: bool,
    pub self_signed: bool,
}

impl TlsConfig {
    /// TLS mode is active when ARS_TLS_DOMAIN is set.
    pub fn from_env() -> Option<Self> {
        let domain = std::env::var("ARS_TLS_DOMAIN").ok()?;
        Some(TlsConfig {
            domain,
            email: std::env::var("ARS_TLS_EMAIL").ok(),
            cert_dir: std::env::var("ARS_TLS_CERT_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("./certs")),
            http_port: std::env::var("ARS_TLS_HTTP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(80),
            staging: std::env::var("ARS_TLS_STAGING").is_ok(),
            self_signed: std::env::var("ARS_TLS_SELF_SIGNED").is_ok(),
        })
    }
}

/// Serve `app` over HTTPS on `https_port`, plus HTTP on `cfg.http_port` for
/// ACME challenges and HTTPS redirects. Blocks until the server exits.
pub async fn run(
    app: Router,
    https_port: u16,
    cfg: TlsConfig,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    std::fs::create_dir_all(&cfg.cert_dir)?;
    let store: ChallengeStore = Arc::new(RwLock::new(HashMap::new()));

    spawn_http_server(&cfg, store.clone(), https_port).await?;

    let cert_path = cfg.cert_dir.join("cert.pem");
    let key_path = cfg.cert_dir.join("key.pem");
    if !cert_path.exists() || !key_path.exists() {
        issue_certificate(&cfg, &store).await?;
    }

    if !cfg.self_signed {
        tokio::spawn(renewal_loop(cfg.clone(), store));
    }

    let tls = RustlsConfig::from_pem_file(&cert_path, &key_path).await?;
    let addr = SocketAddr::from(([0, 0, 0, 0], https_port));
    info!("HTTPS server listening on {addr}");
    axum_server::bind_rustls(addr, tls)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

/// Issue (or self-sign) a certificate and write cert.pem, key.pem and the
/// issuance timestamp into cfg.cert_dir.
async fn issue_certificate(
    cfg: &TlsConfig,
    store: &ChallengeStore,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (cert_pem, key_pem) = if cfg.self_signed {
        info!(domain = %cfg.domain, "generating self-signed certificate");
        let rcgen::CertifiedKey { cert, key_pair } =
            rcgen::generate_simple_self_signed(vec![cfg.domain.clone()])?;
        (cert.pem(), key_pair.serialize_pem())
    } else {
        acme_issue(cfg, store).await?
    };

    std::fs::write(cfg.cert_dir.join("cert.pem"), cert_pem)?;
    std::fs::write(cfg.cert_dir.join("key.pem"), key_pem)?;
    std::fs::write(cfg.cert_dir.join("issued"), now_secs().to_string())?;
    info!(domain = %cfg.domain, dir = %cfg.cert_dir.display(), "certificate written");
    Ok(())
}

/// Check daily whether the certificate is due for renewal; re-issue and exit
/// so the container restarts with the fresh certificate.
async fn renewal_loop(cfg: TlsConfig, store: ChallengeStore) {
    loop {
        sleep(RENEW_CHECK_INTERVAL).await;
        if !needs_renewal(&cfg.cert_dir) {
            continue;
        }
        match issue_certificate(&cfg, &store).await {
            Ok(()) => {
                info!("certificate renewed; restarting to load it");
                std::process::exit(0);
            }
            Err(e) => error!("certificate renewal failed: {e}; retrying tomorrow"),
        }
    }
}

fn needs_renewal(cert_dir: &Path) -> bool {
    let Ok(contents) = std::fs::read_to_string(cert_dir.join("issued")) else {
        return true;
    };
    let Ok(issued) = contents.trim().parse::<u64>() else {
        return true;
    };
    now_secs().saturating_sub(issued) > RENEW_AFTER_SECS
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Run an ACME order for cfg.domain using HTTP-01 challenges published in
/// `store`. Returns (cert chain PEM, private key PEM).
async fn acme_issue(
    cfg: &TlsConfig,
    store: &ChallengeStore,
) -> Result<(String, String), Box<dyn Error + Send + Sync>> {
    let account = acme_account(cfg).await?;
    let directory = match cfg.staging {
        true => LetsEncrypt::Staging,
        false => LetsEncrypt::Production,
    };
    info!(domain = %cfg.domain, url = directory.url(), "starting ACME order");

    let identifier = Identifier::Dns(cfg.domain.clone());
    let mut order = account.new_order(&NewOrder::new(&[identifier])).await?;

    let result = complete_order(&mut order, store).await;
    store.write().unwrap().clear();
    result
}

async fn complete_order(
    order: &mut Order,
    store: &ChallengeStore,
) -> Result<(String, String), Box<dyn Error + Send + Sync>> {
    let mut authorizations = order.authorizations();
    while let Some(result) = authorizations.next().await {
        let mut authz = result?;
        match authz.status {
            AuthorizationStatus::Pending => {}
            AuthorizationStatus::Valid => continue,
            status => return Err(format!("authorization not usable: {status:?}").into()),
        }
        let mut challenge = authz
            .challenge(ChallengeType::Http01)
            .ok_or("no http-01 challenge offered")?;
        store.write().unwrap().insert(
            challenge.token.clone(),
            challenge.key_authorization().as_str().to_string(),
        );
        challenge.set_ready().await?;
    }
    drop(authorizations);

    // Exponentially back off until the order becomes ready or invalid.
    let mut tries = 1u8;
    let mut delay = Duration::from_millis(250);
    loop {
        sleep(delay).await;
        let state = order.refresh().await?;
        if let OrderStatus::Ready | OrderStatus::Invalid = state.status {
            break;
        }
        delay *= 2;
        tries += 1;
        if tries >= 8 {
            return Err(format!("order not ready after {tries} tries: {state:#?}").into());
        }
    }
    if order.state().status != OrderStatus::Ready {
        return Err(format!("unexpected order status: {:?}", order.state().status).into());
    }

    let private_key_pem = order.finalize().await?;
    let mut tries = 0u8;
    let cert_chain_pem = loop {
        match order.certificate().await? {
            Some(pem) => break pem,
            None if tries < 30 => {
                tries += 1;
                sleep(Duration::from_secs(1)).await;
            }
            None => return Err("certificate still processing after 30s".into()),
        }
    };
    Ok((cert_chain_pem, private_key_pem))
}

/// Load the ACME account from disk, or create and persist a new one.
async fn acme_account(cfg: &TlsConfig) -> Result<Account, Box<dyn Error + Send + Sync>> {
    let path = cfg.cert_dir.join("account.json");
    let directory = match cfg.staging {
        true => LetsEncrypt::Staging,
        false => LetsEncrypt::Production,
    };

    if let Ok(contents) = std::fs::read_to_string(&path) {
        match serde_json::from_str::<AccountCredentials>(&contents) {
            Ok(credentials) => {
                if let Ok(account) = Account::builder()?.from_credentials(credentials).await {
                    return Ok(account);
                }
            }
            Err(e) => error!("invalid {path:?}: {e}; creating a new ACME account"),
        }
    }

    let contact: Vec<String> = cfg
        .email
        .as_ref()
        .map(|e| match e.starts_with("mailto:") {
            true => vec![e.clone()],
            false => vec![format!("mailto:{e}")],
        })
        .unwrap_or_default();
    let contact_refs: Vec<&str> = contact.iter().map(String::as_str).collect();
    let (account, credentials) = Account::builder()?
        .create(
            &NewAccount {
                contact: &contact_refs,
                terms_of_service_agreed: true,
                only_return_existing: false,
            },
            directory.url().to_string(),
            None,
        )
        .await?;
    std::fs::write(&path, serde_json::to_string_pretty(&credentials)?)?;
    Ok(account)
}

/// HTTP listener: serves ACME HTTP-01 challenges and redirects everything
/// else to HTTPS.
async fn spawn_http_server(
    cfg: &TlsConfig,
    store: ChallengeStore,
    https_port: u16,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    #[derive(Clone)]
    struct HttpState {
        store: ChallengeStore,
        location_prefix: String,
    }

    async fn challenge(
        AxumPath(token): AxumPath<String>,
        State(state): State<HttpState>,
    ) -> Result<String, StatusCode> {
        state
            .store
            .read()
            .unwrap()
            .get(&token)
            .cloned()
            .ok_or(StatusCode::NOT_FOUND)
    }

    async fn redirect(State(state): State<HttpState>, uri: Uri) -> Redirect {
        let path = uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("/");
        Redirect::permanent(&format!("{}{}", state.location_prefix, path))
    }

    let port_suffix = match https_port {
        443 => String::new(),
        port => format!(":{port}"),
    };
    let state = HttpState {
        store,
        location_prefix: format!("https://{}{}", cfg.domain, port_suffix),
    };

    let router = Router::new()
        .route("/.well-known/acme-challenge/{token}", get(challenge))
        .fallback(redirect)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.http_port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("HTTP challenge/redirect server listening on {addr}");
    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            error!("HTTP challenge/redirect server failed: {e}");
        }
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_cert_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ars_tls_test_{}_{}", tag, std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_needs_renewal() {
        let dir = temp_cert_dir("renewal");

        // Missing issued file: renew
        assert!(needs_renewal(&dir));

        // Freshly issued: no renewal
        std::fs::write(dir.join("issued"), now_secs().to_string()).unwrap();
        assert!(!needs_renewal(&dir));

        // Issued 61 days ago: renew
        let old = now_secs() - 61 * 24 * 60 * 60;
        std::fs::write(dir.join("issued"), old.to_string()).unwrap();
        assert!(needs_renewal(&dir));

        // Garbage contents: renew
        std::fs::write(dir.join("issued"), "not-a-number").unwrap();
        assert!(needs_renewal(&dir));

        std::fs::remove_dir_all(&dir).ok();
    }
}
