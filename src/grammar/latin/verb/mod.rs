use crate::grammar::latin::Number;
use crate::schema::latin_verbs::{self};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use strum_macros::{Display, EnumString};
use unaccent::unaccent as local_unaccent;
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
#[diesel(treat_none_as_default_value = false)]
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

    pub fn esse(&self) -> Verb {
        Verb {
            id: None,
            conjugation: Conjugation::Esse,
            present: "sum".to_string(),
            infinitive: "esse".to_string(),
            perfect: "fuī".to_string(),
            supine: Some("futūrum".to_string()),
        }
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
        if self.verb.is_deponent() {
            self.voice = Voice::Passive
        }
        match self.verb.conjugation {
            Conjugation::I => self.conjugate_i(),
            Conjugation::II => self.conjugate_ii(),
            Conjugation::III => self.conjugate_iii(),
            Conjugation::IV => self.conjugate_iv(),
            Conjugation::Esse => self.conjugate_esse(),
            _ => panic!("Not implemented"), // TODO: irregular
        }
    }

    fn get_stem(&self) -> String {
        if self.mood == Mood::Subjunctive && self.tense == Tense::Imperfect {
            return self
                .verb
                .infinitive
                .chars()
                .take(self.verb.infinitive.chars().count() - 1)
                .collect();
        } else if self.tense == Tense::Perfect && self.voice == Voice::Passive {
            return self
                .verb
                .supine
                .as_ref()
                .filter(|s| !s.is_empty())
                .cloned()
                .unwrap_or(self.verb.perfect.to_string());
        }

        let deponent = self.verb.is_deponent();
        let (prinicpal_part, ch) = match self.tense {
            Tense::Present | Tense::Imperfect | Tense::Future => match deponent {
                false => (self.verb.present.clone(), 1),
                true => (self.verb.present.clone(), 2),
            },
            Tense::Perfect => match self.voice {
                Voice::Active => (self.verb.perfect.clone(), 1),
                Voice::Passive => match deponent {
                    false => (self.verb.perfect.clone(), 1),
                    true => (
                        self.verb
                            .supine
                            .as_ref()
                            .unwrap_or(&self.verb.perfect)
                            .to_string(),
                        0,
                    ),
                },
            },
        };

        prinicpal_part
            .clone()
            .chars()
            .take(prinicpal_part.chars().count() - ch)
            .collect()
    }

    fn match_stem_vowel(&self) -> (&str, &str) {
        match self.verb.conjugation {
            Conjugation::I => ("ā", "ē"),
            Conjugation::II => ("ē", "ā"),
            _ => ("", ""),
        }
    }

    fn get_stem_vowel(&self, stem_vowel_ind: &str, stem_vowel_sub: &str) -> String {
        if self.tense == Tense::Perfect && (self.verb.is_deponent() || self.voice == Voice::Passive)
        {
            return " ".to_string();
        }
        let stem_vowel = if self.mood == Mood::Subjunctive {
            stem_vowel_sub.to_string()
        } else if self.tense == Tense::Perfect {
            "ī".to_string()
        } else {
            stem_vowel_ind.to_string()
        };
        match (self.mood, self.tense) {
            (Mood::Indicative, Tense::Imperfect | Tense::Future) => stem_vowel,
            (_, Tense::Perfect) => match self.voice {
                Voice::Active => match self.mood {
                    Mood::Indicative => match (self.person, self.number) {
                        (Person::First, Number::Singular) => stem_vowel,
                        (Person::Third, Number::Plural) => "ēru".to_string(),
                        _ => local_unaccent(&stem_vowel).to_string(),
                    },
                    Mood::Subjunctive => "eri".to_string(),
                },
                _ => "".to_string(),
            },
            _ => match (self.person, self.number) {
                (Person::Third, _) => match (self.voice, self.number) {
                    (Voice::Passive, Number::Singular) => stem_vowel,
                    _ => local_unaccent(&stem_vowel).to_string(),
                },
                (Person::First, Number::Singular) => match self.mood {
                    Mood::Indicative => "".to_string(),
                    Mood::Subjunctive => local_unaccent(&stem_vowel).to_string(),
                },
                (Person::Second, _) => stem_vowel,
                _ => stem_vowel,
            },
        }
    }
    fn handle_deponent(&self) -> String {
        let tense = match self.tense {
            Tense::Perfect => Tense::Present,
            _ => Tense::Future,
        };

        let esse = self.verb.esse();
        let mut esse_instance = VerbInstance {
            verb: &esse,
            tense,
            ..*self
        };
        println!("{}", esse_instance.conjugate());
        esse_instance.conjugate()
    }

    fn get_ending(&self) -> &'static str {
        match self.person {
            Person::First => match self.number {
                Number::Singular => match self.mood {
                    Mood::Indicative => match self.voice {
                        Voice::Active => match self.tense {
                            Tense::Present | Tense::Future => "ō",
                            Tense::Imperfect => "m",
                            Tense::Perfect => "",
                        },
                        Voice::Passive => "or",
                    },
                    Mood::Subjunctive => "m",
                },
                Number::Plural => match self.voice {
                    Voice::Active => "mus",
                    Voice::Passive => "mur",
                },
            },
            Person::Second => match self.number {
                Number::Singular => match self.voice {
                    Voice::Active => match (self.tense, self.mood) {
                        (Tense::Perfect, Mood::Indicative) => "stī",
                        _ => "s",
                    },
                    Voice::Passive => "ris",
                },
                Number::Plural => match self.voice {
                    Voice::Active => match (self.mood, self.tense) {
                        (Mood::Indicative, Tense::Perfect) => "stis",
                        _ => "tis",
                    },
                    Voice::Passive => "minī",
                },
            },
            Person::Third => match self.number {
                Number::Singular => match self.voice {
                    Voice::Active => "t",
                    Voice::Passive => "tur",
                },
                Number::Plural => match self.voice {
                    Voice::Active => "nt",
                    Voice::Passive => "ntur",
                },
            },
        }
    }

    pub fn infinitive(&mut self) -> String {
        if self.verb.is_deponent() {
            self.voice = Voice::Passive
        }
        match self.tense {
            Tense::Present => match self.voice {
                Voice::Active => self.verb.infinitive.clone(),
                Voice::Passive => {
                    self.verb
                        .infinitive
                        .chars()
                        .take(self.verb.infinitive.chars().count() - 1)
                        .collect::<String>()
                        + "ī"
                }
            },
            Tense::Perfect => match self.voice {
                Voice::Active => {
                    self.verb
                        .perfect
                        .chars()
                        .take(self.verb.perfect.chars().count() - 1)
                        .collect::<String>()
                        + "isse"
                }
                // Voice::Passive => self.verb.supine.clone() + "esse",
                // Voice::Passive => self.verb.supine.to_string() + "esse",
                Voice::Passive => format!(
                    "{} esse",
                    self.verb
                        .supine
                        .as_deref()
                        .filter(|s| !s.is_empty())
                        .unwrap_or(&self.verb.perfect.to_string())
                ),
            },
            _ => "".to_string(),
            // _ => panic!("Not implemented"),
        }
    }
}
