use crate::establish_cnx;
use crate::{
    api::latin::{WordResult, lookup_word_cnx},
    grammar::latin::{
        adjective::Adjective,
        noun::{Case, Noun},
        verb::{Mood, Person, Tense, Verb, Voice},
    },
};

pub mod word_order;

pub enum PoS {
    VERB,
    NOUN,
    PRONOUN,
    ADJECTIVE,
    ADVERB,
    CONJUNCTION,
    PREPOSITION,
    INTERJECTION,
}

pub enum GrammaticalRole {
    Subject,
    DirectObject,
    IndirectObject,
    Verb,
    Adjective,
    Preposition,
    Conjunction,
}

pub enum Token {
    Verb {
        verb: Verb,
        person: Person,
        number: crate::grammar::latin::Number,
        tense: Tense,
        mood: Mood,
        voice: Voice,
        infinitive: bool,
    },
    Noun {
        noun: Noun,
        case: Case,
        number: crate::grammar::latin::Number,
    },
    Adjective {
        adjective: Adjective,
        case: Case,
        number: crate::grammar::latin::Number,
        gender: crate::grammar::latin::noun::Gender,
    },
}

pub struct Word {
    pub token: Token,
    pub inflected: String,
    pub path: i32,
    pub role: GrammaticalRole,
}

pub struct Text {
    pub title: String,
    pub author: String,
    pub words: Vec<String>,
    pub parsed: Vec<Word>,
}

impl Text {
    pub fn read_text(path: &str, title: &str, author: &str) -> std::io::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let words: Vec<String> = contents
            .split(|c: char| !c.is_alphabetic())
            .filter(|w| !w.is_empty())
            .map(|w| w.to_lowercase())
            .collect();
        Ok(Text {
            title: title.to_string(),
            author: author.to_string(),
            words,
            parsed: Vec::new(),
        })
    }

    pub fn parse(&mut self) {
        let mut cnx = establish_cnx();
        for word in &self.words {
            if let Ok(results) = lookup_word_cnx(&mut cnx, word, 5) {
                for r in &results {
                    let (token, role) = match r {
                        WordResult::Verb {
                            id,
                            conjugation,
                            present,
                            infinitive,
                            perfect,
                            supine,
                            person,
                            number,
                            tense,
                            mood,
                            voice,
                            infinitive_flag,
                            ..
                        } => (
                            Token::Verb {
                                verb: Verb {
                                    id: Some(*id),
                                    conjugation: *conjugation,
                                    present: present.clone(),
                                    infinitive: infinitive.clone(),
                                    perfect: perfect.clone(),
                                    supine: supine.clone(),
                                },
                                person: *person,
                                number: *number,
                                tense: *tense,
                                mood: *mood,
                                voice: *voice,
                                infinitive: *infinitive_flag,
                            },
                            GrammaticalRole::Verb,
                        ),
                        WordResult::Noun {
                            id,
                            declension,
                            nominative,
                            genitive,
                            gender,
                            case,
                            number,
                            ..
                        } => {
                            let role = match case {
                                Case::Nominative => GrammaticalRole::Subject,
                                Case::Accusative => GrammaticalRole::DirectObject,
                                _ => GrammaticalRole::IndirectObject,
                            };
                            (
                                Token::Noun {
                                    noun: Noun {
                                        id: Some(*id),
                                        declension: *declension,
                                        nominative: nominative.clone(),
                                        genitive: genitive.clone(),
                                        gender: *gender,
                                    },
                                    case: *case,
                                    number: *number,
                                },
                                role,
                            )
                        }
                        WordResult::Adjective {
                            id,
                            declension,
                            f,
                            m,
                            n,
                            case,
                            number,
                            gender,
                            ..
                        } => (
                            Token::Adjective {
                                adjective: Adjective {
                                    id: Some(*id),
                                    declension: *declension,
                                    f: f.clone(),
                                    m: m.clone(),
                                    n: n.clone(),
                                },
                                case: *case,
                                number: *number,
                                gender: *gender,
                            },
                            GrammaticalRole::Adjective,
                        ),
                        _ => continue,
                    };

                    let inflected = match r {
                        WordResult::Verb { form, .. }
                        | WordResult::Noun { form, .. }
                        | WordResult::Adjective { form, .. } => form.clone(),
                        _ => continue,
                    };
                    let path = match r {
                        WordResult::Verb { path, .. }
                        | WordResult::Noun { path, .. }
                        | WordResult::Adjective { path, .. } => *path,
                        _ => continue,
                    };

                    self.parsed.push(Word {
                        token,
                        inflected,
                        path,
                        role,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_text() {
        let txt = Text::read_text(
            "./data/latin/test/text.txt",
            "De Bello Civile",
            "C. Caesaris",
        );

        assert_eq!(110, txt.as_ref().unwrap().words.len());

        assert_eq!("litteris", txt.as_ref().unwrap().words.first().unwrap());
        assert_eq!("c", txt.as_ref().unwrap().words[1]);
        assert_eq!("imploraturum", txt.unwrap().words.last().unwrap());
    }
}
