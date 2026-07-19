use crate::{
    api::{app::AppState, unaccent},
    establish_cnx,
    grammar::latin::{
        Number,
        adjective::{AdjDeclension, Adjective},
        noun::{Case, Declension, Gender, Noun},
        path,
        preposition::Preposition,
        verb::{Conjugation, Mood, Person, Tense, Verb, Voice},
        word::{self, LatinPos, LatinWord},
    },
    schema::{
        latin_adjectives, latin_lookup, latin_nouns, latin_prepositions, latin_verbs, latin_words,
    },
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};

use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl};
use uuid::Uuid;

pub mod nouns;
pub mod prepositions;
pub mod verbs;

pub use prepositions::create_latin_preposition;

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(tag = "pos")]
pub enum WordResult {
    Verb {
        id: Uuid,
        conjugation: Conjugation,
        present: String,
        infinitive: String,
        perfect: String,
        supine: Option<String>,
        form: String,
        #[serde(skip)]
        path: i32,
        person: Person,
        number: Number,
        tense: Tense,
        mood: Mood,
        voice: Voice,
        infinitive_flag: bool,
    },
    Noun {
        id: Uuid,
        declension: Declension,
        nominative: String,
        genitive: String,
        gender: Gender,
        form: String,
        #[serde(skip)]
        path: i32,
        case: Case,
        number: Number,
    },
    Adjective {
        id: Uuid,
        declension: AdjDeclension,
        f: String,
        m: String,
        n: String,
        form: String,
        #[serde(skip)]
        path: i32,
        case: Case,
        number: Number,
        gender: Gender,
    },
    Preposition {
        id: Uuid,
        word: String,
        cases: Vec<Case>,
        form: String,
        #[serde(skip)]
        path: i32,
    },
}

pub fn lookup_word_cnx(
    cnx: &mut PgConnection,
    word: &str,
    limit: i64,
) -> Result<Vec<WordResult>, diesel::result::Error> {
    // latin_lookup.form is stored unaccented; a plain btree index serves this
    let lookups: Vec<(Uuid, Uuid, String, i32)> = latin_lookup::table
        .filter(latin_lookup::form.eq(unaccent(word)))
        .limit(limit)
        .select((
            latin_lookup::id,
            latin_lookup::word,
            latin_lookup::form,
            latin_lookup::path,
        ))
        .load(cnx)?;

    let mut results = Vec::new();
    for (_lookup_id, word_id, form, path_val) in lookups {
        let lw = latin_words::table
            .filter(latin_words::id.eq(word_id))
            .select(LatinWord::as_select())
            .first(cnx)?;

        match lw.pos {
            LatinPos::Verb => {
                if let Ok(verb) = latin_verbs::table
                    .filter(latin_verbs::id.eq(word_id))
                    .select(Verb::as_select())
                    .first(cnx)
                {
                    let (person, number, tense, mood, voice, inf) = path::decode_verb(path_val);
                    results.push(WordResult::Verb {
                        id: verb.id.expect("Verb has no id"),
                        conjugation: verb.conjugation,
                        present: verb.present,
                        infinitive: verb.infinitive,
                        perfect: verb.perfect,
                        supine: verb.supine,
                        form,
                        path: path_val,
                        person,
                        number,
                        tense,
                        mood,
                        voice,
                        infinitive_flag: inf,
                    });
                }
            }
            LatinPos::Noun => {
                if let Ok(noun) = latin_nouns::table
                    .filter(latin_nouns::id.eq(word_id))
                    .select(Noun::as_select())
                    .first(cnx)
                {
                    let (case, number) = path::decode_noun(path_val);
                    results.push(WordResult::Noun {
                        id: noun.id.expect("Noun has no id"),
                        declension: noun.declension,
                        nominative: noun.nominative,
                        genitive: noun.genitive,
                        gender: noun.gender,
                        form,
                        path: path_val,
                        case,
                        number,
                    });
                }
            }
            LatinPos::Adjective => {
                if let Ok(adj) = latin_adjectives::table
                    .filter(latin_adjectives::id.eq(word_id))
                    .select(Adjective::as_select())
                    .first(cnx)
                {
                    let (gender, case, number) = path::decode_adjective(path_val);
                    results.push(WordResult::Adjective {
                        id: adj.id.expect("Adjective has no id"),
                        declension: adj.declension,
                        f: adj.f,
                        m: adj.m,
                        n: adj.n,
                        form,
                        path: path_val,
                        case,
                        number,
                        gender,
                    });
                }
            }
            LatinPos::Preposition => {
                if let Ok(prep) = latin_prepositions::table
                    .filter(latin_prepositions::id.eq(word_id))
                    .select(Preposition::as_select())
                    .first(cnx)
                {
                    results.push(WordResult::Preposition {
                        id: prep.id.expect("Preposition has no id"),
                        word: prep.word,
                        cases: prep.cases,
                        form,
                        path: path_val,
                    });
                }
            }
            _ => {}
        }
    }

    Ok(results)
}

/// Look up all known forms of a word
///
/// Returns every dictionary entry matching the given form, along with the
/// grammatical analysis (path encoding) for each match.
#[utoipa::path(
    get,
    path = "/latin/query/{word}",
    params(
        ("word" = String, Path, description = "Inflected word form to look up")
    ),
    responses(
        (status = 200, description = "Matching lexical entries with grammatical analysis", body = [WordResult]),
        (status = 500, description = "Internal server error")
    ),
    tag = "latin"
)]
pub async fn lookup_word(
    State(_state): State<AppState>,
    Path(word): Path<String>,
) -> Result<Json<Vec<WordResult>>, StatusCode> {
    let mut cnx = establish_cnx();
    let results =
        lookup_word_cnx(&mut cnx, &word, 10).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(results))
}

pub fn create_latin_noun(
    cnx: &mut PgConnection,
    noun: &Noun,
) -> Result<Noun, diesel::result::Error> {
    let word_id = word::create_latin_word(cnx, LatinPos::Noun)?;

    diesel::insert_into(latin_nouns::table)
        .values(Noun {
            id: Some(word_id),
            ..noun.clone()
        })
        .returning(Noun::as_returning())
        .get_result(cnx)
}

pub fn create_latin_adjective(
    cnx: &mut PgConnection,
    adjective: &Adjective,
) -> Result<Adjective, diesel::result::Error> {
    let word_id = word::create_latin_word(cnx, LatinPos::Adjective)?;

    diesel::insert_into(latin_adjectives::table)
        .values(Adjective {
            id: Some(word_id),
            ..adjective.clone()
        })
        .returning(Adjective::as_returning())
        .get_result(cnx)
}

pub fn create_latin_verb(
    cnx: &mut PgConnection,
    verb: &Verb,
) -> Result<Verb, diesel::result::Error> {
    let word_id = word::create_latin_word(cnx, LatinPos::Verb)?;

    diesel::insert_into(latin_verbs::table)
        .values(Verb {
            id: Some(word_id),
            ..verb.clone()
        })
        .returning(Verb::as_returning())
        .get_result(cnx)
}
