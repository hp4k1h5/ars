/// Iterate over all latin_db entries, conjugate or decline every form. Store in latin_lookup with
/// bitmap path encoding
use ars::establish_cnx;
use ars::grammar::latin::Number;
use ars::grammar::latin::adjective::{Adjective, AdjectiveInstance};
use ars::grammar::latin::noun::{Case, Gender, Noun, NounInstance};
use ars::grammar::latin::path;
use ars::grammar::latin::preposition::Preposition;
use ars::grammar::latin::verb::{Mood, Person, Tense, Verb, VerbInstance, Voice};
use ars::schema::{latin_adjectives, latin_lookup, latin_nouns, latin_prepositions, latin_verbs};
use diesel::prelude::*;
use std::error::Error;
use strum::IntoEnumIterator;
use uuid::Uuid;

#[derive(Insertable, Debug)]
#[diesel(table_name = latin_lookup)]
struct NewLookupEntry {
    word: Uuid,
    form: String,
    path: i32,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut cnx = establish_cnx();

    compile_verbs(&mut cnx)?;
    compile_nouns(&mut cnx)?;
    compile_adjectives(&mut cnx)?;
    compile_prepositions(&mut cnx)?;

    Ok(())
}

fn insert_lookups(
    cnx: &mut PgConnection,
    entries: Vec<NewLookupEntry>,
) -> Result<(), Box<dyn Error>> {
    tracing::info!("Inserting {} entries", entries.len());
    diesel::insert_into(latin_lookup::table)
        .values(entries)
        .execute(cnx)?;
    Ok(())
}

fn compile_verbs(cnx: &mut PgConnection) -> Result<(), Box<dyn Error>> {
    let verbs: Vec<Verb> = latin_verbs::table.select(Verb::as_select()).load(cnx)?;

    let persons = [Person::First, Person::Second, Person::Third];
    let numbers = [Number::Singular, Number::Plural];
    let tenses = [
        Tense::Present,
        Tense::Imperfect,
        Tense::Future,
        Tense::Perfect,
        Tense::Pluperfect,
        Tense::FuturePerfect,
    ];
    let moods = [Mood::Indicative, Mood::Subjunctive, Mood::Imperative];
    let active_passive = [Voice::Active, Voice::Passive];

    for verb in &verbs {
        let mut entries: Vec<NewLookupEntry> = vec![];
        let verb_id = verb.id.expect("Verb has no id");
        println!("Processing verb: {}, ID: {}", verb.present, verb_id);

        for person in &persons {
            for number in &numbers {
                for tense in &tenses {
                    for mood in &moods {
                        if *mood == Mood::Subjunctive && *tense == Tense::Future {
                            continue;
                        }
                        if *mood == Mood::Imperative && *person != Person::Second {
                            continue;
                        }
                        for voice in &active_passive {
                            if verb.is_deponent() && *voice == Voice::Passive {
                                continue;
                            }
                            let mut instance = VerbInstance {
                                verb,
                                person: *person,
                                number: *number,
                                tense: *tense,
                                mood: *mood,
                                voice: *voice,
                            };

                            let conjugated = instance.conjugate();
                            let bitmap = path::encode_verb(instance, false);

                            entries.push(NewLookupEntry {
                                word: verb_id,
                                form: conjugated.to_string(),
                                path: bitmap,
                            });
                        }
                    }
                }
            }
        }

        // Infinitives
        for tense in [Tense::Present, Tense::Perfect] {
            for voice in &active_passive {
                if verb.is_deponent() && *voice == Voice::Passive {
                    continue;
                }
                let mut instance = VerbInstance {
                    verb,
                    person: Person::First,
                    number: Number::Singular,
                    tense,
                    mood: Mood::Indicative,
                    voice: *voice,
                };
                let infinitive = instance.infinitive();

                let bitmap = path::encode_verb(instance, true);
                entries.push(NewLookupEntry {
                    word: verb_id,
                    form: infinitive,
                    path: bitmap,
                });
            }
        }
        insert_lookups(cnx, entries)?;
    }
    Ok(())
}

fn compile_nouns(cnx: &mut PgConnection) -> Result<(), Box<dyn Error>> {
    let nouns: Vec<Noun> = latin_nouns::table.select(Noun::as_select()).load(cnx)?;

    let numbers = [Number::Singular, Number::Plural];

    for noun in &nouns {
        let mut entries: Vec<NewLookupEntry> = vec![];
        let noun_id = noun.id.expect("Noun has no id");
        println!("Processing noun: {}, ID: {}", noun.nominative, noun_id);

        for case in Case::iter() {
            for number in &numbers {
                let instance = NounInstance {
                    noun,
                    case,
                    number: *number,
                };
                let declined = instance.decline();
                let bitmap = path::encode_noun(case, *number);
                entries.push(NewLookupEntry {
                    word: noun_id,
                    form: declined,
                    path: bitmap,
                });
            }
        }
        insert_lookups(cnx, entries)?;
    }
    Ok(())
}

fn compile_prepositions(cnx: &mut PgConnection) -> Result<(), Box<dyn Error>> {
    let preps: Vec<Preposition> = latin_prepositions::table
        .select(Preposition::as_select())
        .load(cnx)?;

    let mut entries = vec![];
    for prep in &preps {
        let prep_id = prep.id.expect("Preposition has no id");
        println!("Processing preposition: {}, ID: {}", prep.word, prep_id);

        let bitmap = path::encode_preposition();
        entries.push(NewLookupEntry {
            word: prep_id,
            form: prep.word.clone(),
            path: bitmap,
        });
    }
    insert_lookups(cnx, entries)?;
    Ok(())
}

fn compile_adjectives(cnx: &mut PgConnection) -> Result<(), Box<dyn Error>> {
    let adjectives: Vec<Adjective> = latin_adjectives::table
        .select(Adjective::as_select())
        .load(cnx)?;

    let numbers = [Number::Singular, Number::Plural];
    let genders = [Gender::Feminine, Gender::Masculine, Gender::Neuter];

    for adj in &adjectives {
        let mut entries: Vec<NewLookupEntry> = vec![];
        let adj_id = adj.id.expect("Adjective has no id");
        println!(
            "Processing adjective: {}/{}/{}, ID: {}",
            adj.f, adj.m, adj.n, adj_id
        );

        for case in Case::iter() {
            for number in &numbers {
                for gender in &genders {
                    let instance = AdjectiveInstance {
                        adjective: adj,
                        case,
                        number: *number,
                        gender: *gender,
                    };

                    let declined = instance.decline();
                    let bitmap = path::encode_adjective(*gender, case, *number);
                    entries.push(NewLookupEntry {
                        word: adj_id,
                        form: declined,
                        path: bitmap,
                    });
                }
            }
        }

        insert_lookups(cnx, entries)?;
    }

    Ok(())
}
