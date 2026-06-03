use ars::api::{
    app,
    latin::nouns::{decline_noun, search_nouns},
    latin::verbs::{conjugate_verb, search_verbs},
    middleware::log_requests,
};
use axum::{Router, middleware, response::Json, routing::get};
use serde_json::{Value, json};
use tracing::info;

const DEFAULT_PORT: u16 = 7357;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber with JSON formatting for structured logging
    tracing_subscriber::fmt().with_target(false).json().init();

    // Parse port from command line arguments
    let args: Vec<String> = std::env::args().collect();
    let port = if args.len() > 1 {
        args[1].parse::<u16>().unwrap_or_else(|_| {
            eprintln!("Invalid port number, using default {}", DEFAULT_PORT);
            DEFAULT_PORT
        })
    } else {
        DEFAULT_PORT
    };

    let nouns_routes = Router::new()
        .route("/latin/nouns", get(search_nouns))
        .route("/latin/nouns/{noun}/decline", get(decline_noun))
        .with_state(app::AppState {});

    let verbs_routes = Router::new()
        .route("/latin/verbs", get(search_verbs))
        .route("/latin/verbs/{verb}/conjugate", get(conjugate_verb))
        .with_state(app::AppState {});

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .merge(nouns_routes)
        .merge(verbs_routes)
        .layer(middleware::from_fn(log_requests));

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "ars.wiki - Language API"
}

async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "ars.wiki"
    }))
}
