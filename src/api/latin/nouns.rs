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

#[derive(Debug, Deserialize)]
pub struct NounQuery {
    nominative: Option<String>,
    limit: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeclensionQuery {
    case: Option<String>,
    number: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct DeclensionResult {
    declension: Option<String>,
    case: String,
    number: Option<String>,
    declined: String,
}

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
