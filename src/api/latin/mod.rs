use crate::grammar::latin::adjective::{AdjDeclension, Adjective};
use crate::grammar::latin::noun::{Declension, Gender, Noun};
use crate::grammar::latin::verb::{Conjugation, Verb};
use crate::grammar::latin::word::{self, LatinPos};
use crate::schema::{latin_adjectives, latin_nouns, latin_verbs};

use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::{PgConnection, RunQueryDsl};

pub mod nouns;
pub mod prepositions;
pub mod verbs;

pub use prepositions::create_latin_preposition;

pub fn create_latin_noun(
    cnx: &mut PgConnection,
    decl: &Declension,
    nom: &str,
    geni: &str,
    gend: &Gender,
) -> Result<Noun, diesel::result::Error> {
    let word_id = word::create_latin_word(cnx, LatinPos::Noun)?;

    diesel::insert_into(latin_nouns::table)
        .values((
            latin_nouns::id.eq(word_id),
            latin_nouns::declension.eq(*decl),
            latin_nouns::nominative.eq(nom),
            latin_nouns::genitive.eq(geni),
            latin_nouns::gender.eq(*gend),
        ))
        .returning(Noun::as_returning())
        .get_result(cnx)
}

pub fn create_latin_adjective(
    cnx: &mut PgConnection,
    decl: &AdjDeclension,
    f: &str,
    m: &str,
    n: &str,
) -> Result<Adjective, diesel::result::Error> {
    let word_id = word::create_latin_word(cnx, LatinPos::Adjective)?;

    diesel::insert_into(latin_adjectives::table)
        .values((
            latin_adjectives::id.eq(word_id),
            latin_adjectives::declension.eq(*decl),
            latin_adjectives::f.eq(f),
            latin_adjectives::m.eq(m),
            latin_adjectives::n.eq(n),
        ))
        .returning(Adjective::as_returning())
        .get_result(cnx)
}

pub fn create_latin_verb(
    cnx: &mut PgConnection,
    conj: &Conjugation,
    pres: &str,
    inf: &str,
    perf: &str,
    sup: &str,
) -> Result<Verb, diesel::result::Error> {
    let word_id = word::create_latin_word(cnx, LatinPos::Verb)?;

    diesel::insert_into(latin_verbs::table)
        .values((
            latin_verbs::id.eq(word_id),
            latin_verbs::conjugation.eq(*conj),
            latin_verbs::present.eq(pres),
            latin_verbs::infinitive.eq(inf),
            latin_verbs::perfect.eq(perf),
            latin_verbs::supine.eq(sup),
        ))
        .returning(Verb::as_returning())
        .get_result(cnx)
}
