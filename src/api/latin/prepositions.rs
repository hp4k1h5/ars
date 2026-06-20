use crate::{
    api::{app::AppState, unaccent},
    establish_cnx,
    grammar::latin::{
        noun::Case,
        preposition::{NewPreposition, Preposition},
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
use uuid::Uuid;

pub fn create_latin_preposition(
    cnx: &mut PgConnection,
    word: &str,
    cases: &[Case],
) -> Result<Preposition, diesel::result::Error> {
    let word_id = word::create_latin_word(cnx, LatinPos::Preposition)?;

    let new_prep = NewPreposition {
        word: word.to_string(),
        cases: cases.to_vec(),
    };

    diesel::insert_into(latin_prepositions::table)
        .values((
            latin_prepositions::id.eq(word_id),
            latin_prepositions::word.eq(&new_prep.word),
            latin_prepositions::cases.eq(&new_prep.cases),
        ))
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
