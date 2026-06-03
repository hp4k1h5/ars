use crate::{
    api::app::AppState,
    establish_cnx,
    grammar::{
        self,
        latin::{
            Number,
            verb::{Mood, Person, Tense, Verb, VerbInstance, Voice},
        },
    },
    schema::{latin_verbs, unaccent},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use diesel::expression_methods::NullableExpressionMethods;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

/// Search for verbs via a principal part
/// as query_param ?principal_part=verb
/// ex: ?supine=actum
#[derive(Debug, Deserialize)]
pub struct VerbQuery {
    present: Option<String>,
    infinitive: Option<String>,
    perfect: Option<String>,
    supine: Option<String>,
    limit: Option<i64>,
}

pub async fn search_verbs(
    State(_state): State<AppState>,
    Query(params): Query<VerbQuery>,
) -> Result<Json<Vec<Verb>>, StatusCode> {
    let mut cnx = establish_cnx();

    let mut query = latin_verbs::table.into_boxed();

    // Apply filters based on provided query parameters
    if let Some(present) = params.present {
        query = query.filter(unaccent(latin_verbs::present).eq(unaccent(present)));
    } else if let Some(inf) = params.infinitive {
        query = query.filter(unaccent(latin_verbs::infinitive).eq(unaccent(inf)));
    } else if let Some(perfect) = params.perfect {
        query = query.filter(unaccent(latin_verbs::perfect).eq(unaccent(perfect)));
    } else if let Some(supine) = params.supine {
        query = query.filter(unaccent(latin_verbs::supine.assume_not_null()).eq(unaccent(supine)));
    }

    let limit = params.limit.unwrap_or(10);
    let verbs = query
        .limit(limit)
        .select(Verb::as_select())
        .load(&mut cnx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(verbs))
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConjugationQuery {
    person: Option<String>,
    number: Option<String>,
    tense: Option<String>,
    mood: Option<String>,
    voice: Option<String>,
    infinitive: Option<bool>,
}

#[derive(Debug, serde::Serialize)]
pub struct ConjugationResult {
    person: Option<String>,
    number: Option<String>,
    tense: String,
    mood: Option<String>,
    voice: String,
    conjugated: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    infinitive: Option<bool>,
}

pub fn infinitive_verb(verb: &Verb, params: &ConjugationQuery) -> Vec<ConjugationResult> {
    let tenses = get_tenses(params);
    let voices = get_voices(params);

    // Generate all combinations
    let mut results = Vec::new();
    for tense in &tenses {
        for voice in &voices {
            let instance = VerbInstance {
                verb,
                person: Person::First,
                number: Number::Singular,
                tense: *tense,
                mood: Mood::Indicative,
                voice: *voice,
            };

            let conjugated = instance.infinitive();

            results.push(ConjugationResult {
                person: None,
                number: None,
                tense: format!("{:?}", tense),
                mood: None,
                voice: format!("{:?}", voice),
                conjugated,
                infinitive: Some(true),
            });
        }
    }
    results
}

pub async fn conjugate_verb(
    State(_state): State<AppState>,
    Path(verb_id): Path<Uuid>,
    Query(params): Query<ConjugationQuery>,
) -> Result<Json<Vec<ConjugationResult>>, StatusCode> {
    let cnx = establish_cnx();

    let verb = get_verb(cnx, verb_id)?;

    // Parse optional parameters
    let persons = match params.person.as_ref().map(|c| c.to_lowercase()).as_deref() {
        Some("first") => vec![Person::First],
        Some("second") => vec![Person::Second],
        Some("third") => vec![Person::Third],
        None => vec![Person::First, Person::Second, Person::Third],
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let numbers = match params.number.as_ref().map(|c| c.to_lowercase()).as_deref() {
        Some("singular") => vec![Number::Singular],
        Some("plural") => vec![Number::Plural],
        None => vec![Number::Singular, Number::Plural],
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let moods = match params.mood.as_ref().map(|c| c.to_lowercase()).as_deref() {
        Some("indicative") => vec![Mood::Indicative],
        Some("subjunctive") => vec![Mood::Subjunctive],
        None => vec![Mood::Indicative, Mood::Subjunctive],
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let tenses = get_tenses(&params);
    let voices = get_voices(&params);
    let inf = params.infinitive.unwrap_or(false);

    let mut results = Vec::new();

    // Generate requested infinitives
    if inf {
        let _inf = infinitive_verb(&verb, &params);
        results.extend(_inf);
    }

    // Generate all requested combinations
    for person in &persons {
        for number in &numbers {
            for tense in &tenses {
                for mood in &moods {
                    if mood == &Mood::Subjunctive && tense == &Tense::Future {
                        continue;
                    }
                    for voice in &voices {
                        let mut instance = VerbInstance {
                            verb: &verb,
                            person: *person,
                            number: *number,
                            tense: *tense,
                            mood: *mood,
                            voice: *voice,
                        };

                        let conjugated = instance.conjugate();

                        results.push(ConjugationResult {
                            person: Some(format!("{:?}", person)),
                            number: Some(format!("{:?}", number)),
                            tense: format!("{:?}", tense),
                            mood: Some(format!("{:?}", mood)),
                            voice: format!("{:?}", voice),
                            conjugated,
                            infinitive: None,
                        });
                    }
                }
            }
        }
    }

    Ok(Json(results))
}

fn get_verb(mut cnx: PgConnection, verb_id: Uuid) -> Result<Verb, StatusCode> {
    let verb = latin_verbs::table
        .filter(latin_verbs::id.eq(verb_id))
        .select(Verb::as_select())
        .first(&mut cnx)
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(verb)
}

fn get_tenses(params: &ConjugationQuery) -> Vec<grammar::latin::verb::Tense> {
    match params.tense.as_ref().map(|c| c.to_lowercase()).as_deref() {
        Some("present") => vec![Tense::Present],
        Some("imperfect") => vec![Tense::Imperfect],
        Some("future") => vec![Tense::Future],
        Some("perfect") => vec![Tense::Perfect],
        None | _ => vec![
            Tense::Present,
            Tense::Imperfect,
            Tense::Future,
            Tense::Perfect,
        ],
    }
}

fn get_voices(params: &ConjugationQuery) -> Vec<grammar::latin::verb::Voice> {
    match params.voice.as_ref().map(|c| c.to_lowercase()).as_deref() {
        Some("active") => vec![Voice::Active],
        Some("passive") => vec![Voice::Passive],
        None | _ => vec![Voice::Active, Voice::Passive],
    }
}
