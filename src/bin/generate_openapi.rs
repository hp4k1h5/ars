use ars::api::openapi::ApiDoc;
use std::error::Error;
use utoipa::OpenApi;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "docs/openapi.json".to_string());
    let json = ApiDoc::openapi().to_pretty_json()?;
    std::fs::write(&path, format!("{json}\n"))?;
    println!("wrote OpenAPI spec to {path}");
    Ok(())
}
