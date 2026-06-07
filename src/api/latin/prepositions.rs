use crate::{
    api::{app::AppState, latin::prepositions, unaccent},
    establish_cnx,
    grammar::latin::{
        noun::Case,
        preposition::{NewPreposition, Preposition},
    },
    schema::latin_prepositions,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use diesel::prelude::*;

use diesel::prelude::SelectableHelper;
use diesel::{PgConnection, RunQueryDsl};
use serde::Deserialize;
use strum::IntoEnumIterator;
use uuid::Uuid;

pub fn create_latin_preposition(
    cnx: &mut PgConnection,
    word: &str,
    cases: &[Case],
) -> Result<Preposition, diesel::result::Error> {
    let new_prep = NewPreposition {
        word: word.to_string(),
        cases: cases.to_vec(),
    };

    diesel::insert_into(latin_prepositions::table)
        .values(&new_prep)
        .returning(Preposition::as_returning())
        .get_result(cnx)
}

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
