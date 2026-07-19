use ars::api::{
    app::{AppState, health, root},
    latin::{
        lookup_word,
        nouns::{decline_noun, search_nouns},
        prepositions::search_prepositions,
        verbs::{conjugate_verb, search_verbs},
    },
    middleware::log_requests,
    openapi::ApiDoc,
};
use axum::{Router, http::Method, middleware, routing::get};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const DEFAULT_PORT: u16 = 7357;
const DEFAULT_TLS_PORT: u16 = 443;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().with_target(false).init();
    let _ = rustls::crypto::ring::default_provider().install_default();

    let port_arg = std::env::args().nth(1).map(|arg| {
        arg.parse::<u16>().unwrap_or_else(|_| {
            tracing::error!("Invalid port number, using default {}", DEFAULT_PORT);
            DEFAULT_PORT
        })
    });

    let app = build_app();

    match ars::tls::TlsConfig::from_env() {
        Some(cfg) => {
            let port = port_arg.unwrap_or(DEFAULT_TLS_PORT);
            info!(domain = %cfg.domain, "TLS enabled");
            ars::tls::run(app, port, cfg).await.unwrap();
        }
        None => {
            let port = port_arg.unwrap_or(DEFAULT_PORT);
            let addr = format!("0.0.0.0:{}", port);
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

            info!("Server listening on {}", listener.local_addr().unwrap());

            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        }
    }
}

fn build_app() -> Router {
    let nouns_routes = Router::new()
        .route("/latin/nouns", get(search_nouns))
        .route("/latin/nouns/{noun}/decline", get(decline_noun));

    let verbs_routes = Router::new()
        .route("/latin/verbs", get(search_verbs))
        .route("/latin/verbs/{verb}/conjugate", get(conjugate_verb));

    let prepositions_routes = Router::new().route(
        "/latin/prepositions/{preposition}",
        get(search_prepositions),
    );

    // Read-only public API: allow cross-origin GET so the GitHub Pages docs
    // UI can issue try-it-out requests against api.ars.wiki.
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(root))
        .route("/health", get(health))
        .route("/latin/query/{word}", get(lookup_word))
        .merge(nouns_routes)
        .merge(verbs_routes)
        .merge(prepositions_routes)
        .layer(middleware::from_fn(log_requests))
        .layer(cors)
        .with_state(AppState {})
}
