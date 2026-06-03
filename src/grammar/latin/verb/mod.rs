use crate::grammar::latin::Number;
use crate::schema::latin_verbs::{self};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

pub mod esse;
pub mod i;
pub mod ii;
pub mod iii;
pub mod iv;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Conjugation"]
pub enum Conjugation {
    #[db_rename = "I"]
    I = 1,
    #[db_rename = "II"]
    II = 2,
    #[db_rename = "III"]
    III = 3,
    #[db_rename = "IV"]
    IV = 4,
    #[db_rename = "Irr"]
    Irr = 5,
    #[db_rename = "Esse"]
    Esse = 6,
}

#[derive(Debug, Display, EnumString, Clone, Copy, PartialEq)]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, Display, EnumString, Clone, Copy, PartialEq)]
pub enum Tense {
    Present,
    Imperfect,
    Future,
    Perfect,
}

#[derive(Debug, Display, EnumString, Clone, Copy, PartialEq)]
pub enum Mood {
    Indicative,
    Subjunctive,
}

#[derive(Debug, Display, EnumString, Clone, Copy, PartialEq)]
pub enum Voice {
    Active,
    Passive,
}

/// Verb as lexical element
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Queryable, Selectable)]
#[diesel(table_name = latin_verbs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Verb {
    #[diesel(deserialize_as = Uuid)]
    pub id: Option<Uuid>,
    pub conjugation: Conjugation,
    pub present: String,
    pub infinitive: String,
    pub perfect: String,
    pub supine: Option<String>, // past participle
}

impl Verb {
    pub fn is_deponent(&self) -> bool {
        self.present.ends_with("or")
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name= latin_verbs)]
pub struct NewVerb<'a> {
    pub conjugation: Conjugation,
    pub present: &'a str,
    pub infinitive: &'a str,
    pub perfect: &'a str,
    pub supine: &'a str,
}

/// Owned version for CSV deserialization
#[derive(Debug, serde::Deserialize)]
pub struct NewVerbOwned {
    pub conjugation: Conjugation,
    pub present: String,
    pub infinitive: String,
    pub perfect: String,
    pub supine: String,
}

/// Verb as used in a phrase
pub struct VerbInstance<'a> {
    pub verb: &'a Verb,
    pub person: Person,
    pub number: Number,
    pub tense: Tense,
    pub mood: Mood,
    pub voice: Voice,
}

impl VerbInstance<'_> {
    pub fn conjugate(&mut self) -> String {
        let deponent = self.verb.is_deponent();
        if deponent {
            self.voice = Voice::Passive
        }
        match self.verb.conjugation {
            Conjugation::I => self.conjugate_i(),
            Conjugation::II => self.conjugate_ii(),
            Conjugation::III => self.conjugate_iii(),
            Conjugation::IV => self.conjugate_iv(),
            Conjugation::Esse => self.conjugate_esse(),
            _ => panic!("Not implemented"), // TODO: III, IV conjugations
        }
    }

    pub fn infinitive(&self) -> String {
        match self.verb.conjugation {
            Conjugation::I => self.infinitive_i(),
            Conjugation::II => self.infinitive_ii(),
            Conjugation::III => self.infinitive_iii(),
            Conjugation::IV => self.infinitive_iv(),
            // Conjugation::Esse => self.infinitive_esse(),
            _ => panic!("Not implemented"), // TODO: III, IV conjugations
        }
    }
}
