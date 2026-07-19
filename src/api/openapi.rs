use utoipa::OpenApi;

use crate::api::app::HealthResponse;
use crate::api::latin::{WordResult, nouns::DeclensionResult, verbs::ConjugationResult};
use crate::grammar::latin::adjective::Adjective;
use crate::grammar::latin::noun::Noun;
use crate::grammar::latin::preposition::Preposition;
use crate::grammar::latin::verb::Verb;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "ars.wiki Language API",
        version = "0.1.0",
        description = "Latin language API: dictionary lookup, noun declension, verb conjugation, and preposition government."
    ),
    servers((url = "http://api.ars.wiki", description = "production")),
    paths(
        crate::api::app::root,
        crate::api::app::health,
        crate::api::latin::lookup_word,
        crate::api::latin::nouns::search_nouns,
        crate::api::latin::nouns::decline_noun,
        crate::api::latin::verbs::search_verbs,
        crate::api::latin::verbs::conjugate_verb,
        crate::api::latin::prepositions::search_prepositions,
    ),
    components(schemas(
        Noun,
        Verb,
        Adjective,
        Preposition,
        WordResult,
        DeclensionResult,
        ConjugationResult,
        HealthResponse,
    )),
    tags(
        (name = "latin", description = "Latin language endpoints"),
        (name = "meta", description = "Service metadata endpoints")
    )
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openapi_doc_covers_all_routes() {
        let doc = ApiDoc::openapi();
        let json = doc.to_json().expect("OpenAPI doc serializes to JSON");

        for path in [
            "/",
            "/health",
            "/latin/query/{word}",
            "/latin/nouns",
            "/latin/nouns/{noun}/decline",
            "/latin/verbs",
            "/latin/verbs/{verb}/conjugate",
            "/latin/prepositions/{preposition}",
        ] {
            assert!(
                json.contains(&format!("\"{path}\"")),
                "missing path {path} in OpenAPI doc"
            );
        }

        assert!(
            json.contains("\"url\":\"http://api.ars.wiki\""),
            "missing production server in OpenAPI doc"
        );
    }
}
