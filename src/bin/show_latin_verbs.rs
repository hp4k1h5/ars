use ars::{
    establish_cnx,
    grammar::latin::{
        Number,
        verb::{Mood, Person, Tense, Verb, VerbInstance, Voice},
    },
};
use diesel::prelude::*;

fn main() {
    use ars::schema::latin_verbs::dsl::*;

    let cnx = &mut establish_cnx();
    let results = latin_verbs
        .limit(4)
        .select(Verb::as_select())
        .load(cnx)
        .expect("Error loading verbs");

    println!("Displaying {} Verbs", results.len());
    for verb in results {
        println!("{:?}", verb.conjugation);
        println!("{}", verb.present);
        println!("{}", verb.infinitive);
        println!("{}", verb.perfect);
        println!("---");

        let mut vi = VerbInstance {
            verb: &verb,
            person: Person::Third,
            number: Number::Plural,
            tense: Tense::Imperfect,
            mood: Mood::Indicative,
            voice: Voice::Active,
        };
        let conj = vi.conjugate();
        println!("Conjugated: {}", conj);
    }
}
