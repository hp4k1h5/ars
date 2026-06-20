use crate::grammar::latin::Number;
use crate::grammar::latin::adjective::{Adjective, AdjectiveInstance};
use crate::grammar::latin::noun::Case;
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
    Pluperfect,
    FuturePerfect,
}

#[derive(Debug, Display, EnumString, Clone, Copy, PartialEq)]
pub enum Mood {
    Indicative,
    Subjunctive,
    Imperative,
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
            Conjugation::I | Conjugation::II => self.conjugate_i_ii(),
            Conjugation::III => self.conjugate_iii(),
            Conjugation::IV => self.conjugate_iv(),
            Conjugation::Esse => self.conjugate_esse(),
            _ => panic!("Not implemented"), // TODO: irregular
        }
    }

    pub fn conjugate_i_ii(&mut self) -> String {
        let stem = self.get_stem();
        let stem_vowel = self.get_stem_vowel();
        let infix: String = self.get_infix();
        let ending = match (self.tense, self.voice, self.mood) {
            (Tense::Perfect, Voice::Passive, _) => &self.handle_deponent(),
            (Tense::Perfect, Voice::Active, _) => &self.perfect_helper(),
            (Tense::Pluperfect, _, Mood::Subjunctive) => self.get_ending(),
            (Tense::Pluperfect | Tense::FuturePerfect, _, _) => &self.perfect_helper(),
            _ => self.get_ending(),
        };
        println!("{stem} | {stem_vowel} | {infix} | {ending}  ");
        format!("{stem}{stem_vowel}{infix}{ending}")
    }
    fn get_stem(&self) -> String {
        let deponent = self.verb.is_deponent();
        let (prinicpal_part, ch) = match (self.tense, self.mood) {
            (Tense::Imperfect, Mood::Subjunctive) => (self.verb.infinitive.clone(), 1),
            (Tense::Present | Tense::Imperfect | Tense::Future, _) => match deponent {
                false => (
                    self.verb.present.clone(),
                    match self.verb.conjugation {
                        Conjugation::I => 1,
                        Conjugation::II => 2,
                        _ => 2,
                    },
                ),
                true => (self.verb.present.clone(), 2),
            },
            (Tense::Perfect | Tense::Pluperfect | Tense::FuturePerfect, _) => match self.voice {
                Voice::Active => (self.verb.perfect.clone(), 1),
                Voice::Passive => return self.handle_supine(),
            },
        };

        prinicpal_part
            .clone()
            .chars()
            .take(prinicpal_part.chars().count() - ch)
            .collect()
    }

    pub fn handle_supine(&self) -> String {
        let verb = &self
            .verb
            .supine
            .as_ref()
            .filter(|s| !s.is_empty())
            .unwrap_or(&self.verb.perfect);
        let stem: String = verb.chars().take(verb.chars().count() - 2).collect();

        let adjective = Adjective {
            id: None,
            declension: super::adjective::AdjDeclension::I_II,
            f: stem.to_owned() + "a",
            m: stem.to_owned() + "us",
            n: stem.to_owned() + "um",
        };
        let ai = AdjectiveInstance {
            adjective: &adjective,
            case: Case::Nominative,
            number: self.number,
            gender: super::noun::Gender::Neuter, // TODO: accept gender as param
        };

        ai.decline()
    }

    fn match_stem_vowel(&self) -> (&str, &str) {
        match self.verb.conjugation {
            Conjugation::I => ("ā", "ē"),
            Conjugation::II => ("ē", "ā"),
            // Conjugation::III => ("i", "ā"),
            _ => todo!("Integrate III, IV, Irr"),
        }
    }

    fn get_stem_vowel(&self) -> String {
        let (stem_vowel_ind, stem_vowel_sub) = self.match_stem_vowel();
        if [Tense::Perfect, Tense::Pluperfect, Tense::FuturePerfect].contains(&self.tense) {
            if self.verb.is_deponent() || self.voice == Voice::Passive {
                return " ".to_string();
            } else {
                return "".to_string();
            }
        }
        let stem_vowel = if self.mood == Mood::Subjunctive {
            stem_vowel_sub.to_string()
        } else if self.tense == Tense::Perfect {
            "".to_string()
        } else {
            stem_vowel_ind.to_string()
        };
        match (self.mood, self.tense) {
            (Mood::Indicative, Tense::Imperfect | Tense::Future) => stem_vowel,
            (Mood::Subjunctive, Tense::Pluperfect) => {
                "iss".to_string()
                    + match (self.person, self.number) {
                        (Person::Second, _) | (Person::First, Number::Plural) => "ē",
                        _ => "e",
                    }
            }
            (_, Tense::Pluperfect | Tense::FuturePerfect) => "".to_string(),
            _ => match (self.person, self.number) {
                (Person::Third, _) => match (self.voice, self.number) {
                    (Voice::Passive, Number::Singular) => stem_vowel,
                    _ => local_unaccent(&stem_vowel).to_string(),
                },
                (Person::First, Number::Singular) => match self.mood {
                    Mood::Indicative => "".to_string(),
                    Mood::Subjunctive => local_unaccent(&stem_vowel).to_string(),
                    Mood::Imperative => panic!("No imperative first person"),
                },
                (Person::Second, _) => stem_vowel,
                _ => stem_vowel,
            },
        }
    }

    fn get_infix(&self) -> String {
        match (self.tense, self.mood) {
            (Tense::Pluperfect, Mood::Subjunctive) => {
                "iss".to_string()
                    + match (self.person, self.number) {
                        (Person::Second, _) | (Person::First, Number::Plural) => "ē",
                        (_, _) => "e",
                    }
            }
            (_, Mood::Subjunctive) => "".to_string(),
            _ => match self.tense {
                Tense::Imperfect => match (self.person, self.number) {
                    (Person::First, Number::Singular) => "ba".to_string(),
                    (Person::Third, _) => match (self.number, self.voice) {
                        (_, Voice::Active) => "ba".to_string(),
                        (Number::Singular, Voice::Passive) => "bā".to_string(),
                        _ => "ba".to_string(),
                    },
                    _ => "bā".to_string(),
                },
                Tense::Future => match (self.person, self.number) {
                    (Person::First, Number::Singular) => "b".to_string(),
                    (Person::Third, Number::Plural) => "bu".to_string(),
                    _ => "bi".to_string(),
                },
                _ => "".to_string(),
            },
        }
    }

    fn handle_deponent(&self) -> String {
        let tense = match self.tense {
            Tense::Perfect => Tense::Present,
            Tense::Pluperfect => Tense::Imperfect,
            _ => Tense::Future,
        };

        let esse = self.verb.esse();
        let mut esse_instance = VerbInstance {
            verb: &esse,
            tense,
            ..*self
        };
        esse_instance.conjugate()
    }

    fn get_ending(&self) -> &'static str {
        match self.person {
            Person::First => match self.number {
                Number::Singular => match self.mood {
                    Mood::Indicative => match self.voice {
                        Voice::Active => match self.tense {
                            Tense::Present => match self.verb.conjugation {
                                Conjugation::II => "eō",
                                _ => "ō",
                            },
                            Tense::Future => match self.verb.conjugation {
                                Conjugation::III | Conjugation::IV => "m",
                                _ => "ō",
                            },
                            Tense::Imperfect => "m",
                            _ => "",
                        },
                        Voice::Passive => match self.tense {
                            Tense::Imperfect => "r",
                            _ => "or",
                        },
                    },
                    Mood::Subjunctive => match self.voice {
                        Voice::Passive => "r",
                        _ => "m",
                    },
                    Mood::Imperative => panic!("No imperative first person"),
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
                        (Tense::Present, Mood::Imperative) => "",
                        _ => "s",
                    },
                    Voice::Passive => "ris",
                },
                Number::Plural => match self.voice {
                    Voice::Active => match (self.tense, self.mood) {
                        (Tense::Perfect, Mood::Indicative) => "stis",
                        (Tense::Present, Mood::Imperative) => "te",
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

    fn esse_helper(&self) -> String {
        let tense = match self.tense {
            Tense::Pluperfect => Tense::Imperfect,
            _ => Tense::Future,
        };
        let verb = self.verb.esse();
        let mut ei = VerbInstance {
            verb: &verb,
            person: self.person,
            number: self.number,
            tense,
            mood: self.mood,
            voice: self.voice,
        };

        let eic = ei.conjugate();
        match (self.tense, self.mood, self.voice) {
            (_, Mood::Subjunctive, _) => match (self.person, self.number, self.voice) {
                (Person::Third, Number::Plural, Voice::Active) => eic.replace("unt", "int"),
                (Person::First, Number::Singular, Voice::Active) => eic.replace("ō", "im"),
                _ => eic,
            },
            (Tense::FuturePerfect, _, Voice::Active) => match (self.person, self.number) {
                (Person::Third, Number::Plural) => eic.replace("unt", "int"),
                _ => eic,
            },
            _ => eic,
        }
    }

    pub fn perfect_helper(&self) -> String {
        match (self.tense, self.voice) {
            (Tense::Perfect, Voice::Active) => match self.mood {
                Mood::Subjunctive => self.esse_helper(),
                Mood::Indicative => match (self.person, self.number) {
                    (Person::First, Number::Singular) => "ī".to_string(),
                    (Person::Second, Number::Singular) => "istī".to_string(),
                    (Person::Third, Number::Singular) => "it".to_string(),
                    (Person::First, Number::Plural) => "imus".to_string(),
                    (Person::Second, Number::Plural) => "istis".to_string(),
                    (Person::Third, Number::Plural) => "ērunt".to_string(),
                },
                _ => "".to_string(),
            },

            (Tense::Pluperfect | Tense::FuturePerfect, _) => match self.mood {
                Mood::Subjunctive => {
                    "iss".to_string()
                        + match (self.person, self.number) {
                            (Person::Second, _) | (Person::First, Number::Plural) => "ē",
                            _ => "e",
                        }
                }
                Mood::Indicative => self.esse_helper(),
                _ => "".to_string(),
            },
            _ => "".to_string(),
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
        }
    }
}
