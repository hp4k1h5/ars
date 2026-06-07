use crate::grammar::latin::adjective::{AdjDeclension, Adjective, NewAdjective};
use crate::grammar::latin::noun::{Declension, Gender, NewNoun, Noun};
use crate::grammar::latin::verb::{Conjugation, NewVerb, Verb};
use crate::schema::{latin_adjectives, latin_nouns};

use diesel::prelude::SelectableHelper;
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
    let new_noun = NewNoun {
        declension: *decl,
        nominative: nom.to_string(),
        genitive: geni.to_string(),
        gender: *gend,
    };

    diesel::insert_into(latin_nouns::table)
        .values(&new_noun)
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
    let new_adj = NewAdjective {
        declension: *decl,
        f: f.to_string(),
        m: m.to_string(),
        n: n.to_string(),
    };

    diesel::insert_into(latin_adjectives::table)
        .values(&new_adj)
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
    use crate::schema::latin_verbs;

    let new_verb = NewVerb {
        conjugation: *conj,
        present: pres,
        infinitive: inf,
        perfect: perf,
        supine: sup,
    };

    diesel::insert_into(latin_verbs::table)
        .values(&new_verb)
        .returning(Verb::as_returning())
        .get_result(cnx)
}
