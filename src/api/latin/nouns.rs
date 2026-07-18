use crate::{
    api::{app::AppState, unaccent},
    establish_cnx,
    grammar::latin::{
        Number,
        noun::{Case, Noun, NounInstance},
    },
    schema::latin_nouns,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use diesel::prelude::*;
use serde::Deserialize;
use strum::IntoEnumIterator;
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct NounQuery {
    /// Filter by nominative form (accent-insensitive)
    nominative: Option<String>,
    /// Maximum number of results (default 10)
    limit: Option<i64>,
}

#[derive(Debug, Deserialize, Clone, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DeclensionQuery {
    /// Grammatical case: nominative, genitive, dative, accusative, ablative, vocative
    case: Option<String>,
    /// Grammatical number: singular or plural
    number: Option<String>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct DeclensionResult {
    declension: Option<String>,
    case: String,
    number: Option<String>,
    declined: String,
}

/// Search Latin nouns
///
/// Optionally filters by nominative form (accent-insensitive).
#[utoipa::path(
    get,
    path = "/latin/nouns",
    params(NounQuery),
    responses(
        (status = 200, description = "List of matching nouns", body = [Noun]),
        (status = 500, description = "Internal server error")
    ),
    tag = "latin"
)]
pub async fn search_nouns(
    State(_state): State<AppState>,
    Query(params): Query<NounQuery>,
) -> Result<Json<Vec<Noun>>, StatusCode> {
    let mut cnx = establish_cnx();

    let mut query = latin_nouns::table.into_boxed();

    if let Some(nominative) = params.nominative {
        query = query.filter(unaccent(latin_nouns::nominative).eq(unaccent(nominative)));
    }

    let limit = params.limit.unwrap_or(10);
    let nouns = query
        .limit(limit)
        .load::<Noun>(&mut cnx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(nouns))
}

/// Decline a noun
///
/// Returns the requested case/number combinations for the noun, defaulting
/// to every case in both numbers.
#[utoipa::path(
    get,
    path = "/latin/nouns/{noun}/decline",
    params(
        ("noun" = Uuid, Path, description = "Noun ID"),
        DeclensionQuery
    ),
    responses(
        (status = 200, description = "Declined forms", body = [DeclensionResult]),
        (status = 400, description = "Invalid case or number parameter"),
        (status = 500, description = "Noun not found or internal server error")
    ),
    tag = "latin"
)]
pub async fn decline_noun(
    State(_state): State<AppState>,
    Path(noun_id): Path<Uuid>,
    Query(params): Query<DeclensionQuery>,
) -> Result<Json<Vec<DeclensionResult>>, StatusCode> {
    let mut cnx = establish_cnx();

    // Fetch noun by ID
    let noun = latin_nouns::table
        .filter(latin_nouns::id.eq(noun_id))
        .select(Noun::as_select())
        .first(&mut cnx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cases = match params.case.as_ref().map(|c| c.to_lowercase()).as_deref() {
        Some("nominative") => vec![Case::Nominative],
        Some("genitive") => vec![Case::Genitive],
        Some("dative") => vec![Case::Dative],
        Some("ablative") => vec![Case::Ablative],
        Some("vocative") => vec![Case::Vocative],
        _ => Case::iter().collect(),
    };

    let numbers = match params.number.as_ref().map(|n| n.to_lowercase()).as_deref() {
        Some("singular") => vec![Number::Singular],
        Some("plural") => vec![Number::Plural],
        None => vec![Number::Singular, Number::Plural],
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    let mut results = Vec::new();

    for case in &cases {
        for number in &numbers {
            let instance = NounInstance {
                noun: &noun,
                case: *case,
                number: *number,
            };

            let declined = instance.decline();

            results.push(DeclensionResult {
                declension: Some(format!("{:?}", noun.declension)),
                case: format!("{:?}", case),
                number: format!("{:?}", number).into(),
                declined,
            });
        }
    }

    Ok(Json(results))
}
