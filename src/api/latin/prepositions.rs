use crate::{
    api::{app::AppState, unaccent},
    establish_cnx,
    grammar::latin::{
        preposition::Preposition,
        word::{self, LatinPos},
    },
    schema::latin_prepositions,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use diesel::prelude::*;

use diesel::prelude::SelectableHelper;
use diesel::{PgConnection, RunQueryDsl};
// use uuid::Uuid;

pub fn create_latin_preposition(
    cnx: &mut PgConnection,
    preposition: &Preposition,
) -> Result<Preposition, diesel::result::Error> {
    let word_id = word::create_latin_word(cnx, LatinPos::Preposition)?;

    diesel::insert_into(latin_prepositions::table)
        .values(Preposition {
            id: Some(word_id),
            ..preposition.clone()
        })
        .returning(Preposition::as_returning())
        .get_result(cnx)
}

/// Look up a preposition
///
/// Returns the preposition and the cases it governs (accent-insensitive).
#[utoipa::path(
    get,
    path = "/latin/prepositions/{preposition}",
    params(
        ("preposition" = String, Path, description = "Preposition to look up")
    ),
    responses(
        (status = 200, description = "Preposition with governed cases", body = Preposition),
        (status = 500, description = "Preposition not found or internal server error")
    ),
    tag = "latin"
)]
pub async fn search_prepositions(
    State(_state): State<AppState>,
    Path(preposition): Path<String>,
) -> Result<Json<Preposition>, StatusCode> {
    let mut cnx = establish_cnx();
    let prep = latin_prepositions::table
        .filter(unaccent(latin_prepositions::word).eq(unaccent(preposition)))
        .select(Preposition::as_select())
        .first(&mut cnx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(prep))
}
