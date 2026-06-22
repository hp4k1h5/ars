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

fn insert_lookup(
    cnx: &mut PgConnection,
    word_id: Uuid,
    form: &str,
    path_val: i32,
) -> Result<(), Box<dyn Error>> {
    let entry = NewLookupEntry {
        word: word_id,
        form: form.to_string(),
        path: path_val,
    };

    diesel::insert_into(latin_lookup::table)
        .values(&entry)
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
                            let bitmap =
                                path::encode_verb(*person, *number, *tense, *mood, *voice, false);

                            insert_lookup(cnx, verb_id, &conjugated, bitmap)?;
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

                let inf = instance.infinitive();
                let bitmap = path::encode_verb(
                    Person::First,
                    Number::Singular,
                    tense,
                    Mood::Indicative,
                    *voice,
                    true,
                );

                insert_lookup(cnx, verb_id, &inf, bitmap)?;
            }
        }
    }

    Ok(())
}

fn compile_nouns(cnx: &mut PgConnection) -> Result<(), Box<dyn Error>> {
    let nouns: Vec<Noun> = latin_nouns::table.select(Noun::as_select()).load(cnx)?;

    let numbers = [Number::Singular, Number::Plural];

    for noun in &nouns {
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

                insert_lookup(cnx, noun_id, &declined, bitmap)?;
            }
        }
    }

    Ok(())
}

fn compile_prepositions(cnx: &mut PgConnection) -> Result<(), Box<dyn Error>> {
    let preps: Vec<Preposition> = latin_prepositions::table
        .select(Preposition::as_select())
        .load(cnx)?;

    for prep in &preps {
        let prep_id = prep.id.expect("Preposition has no id");
        println!("Processing preposition: {}, ID: {}", prep.word, prep_id);

        let bitmap = path::encode_preposition();
        insert_lookup(cnx, prep_id, &prep.word, bitmap)?;
    }

    Ok(())
}

fn compile_adjectives(cnx: &mut PgConnection) -> Result<(), Box<dyn Error>> {
    let adjectives: Vec<Adjective> = latin_adjectives::table
        .select(Adjective::as_select())
        .load(cnx)?;

    let numbers = [Number::Singular, Number::Plural];
    let genders = [Gender::Feminine, Gender::Masculine, Gender::Neuter];

    for adj in &adjectives {
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

                    insert_lookup(cnx, adj_id, &declined, bitmap)?;
                }
            }
        }
    }

    Ok(())
}
