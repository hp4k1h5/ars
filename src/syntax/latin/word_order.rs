// use crate::grammar::latin::*;
// use std::collections::HashMap;

/// Parts of Speech
pub enum PoS {
    NOUN,
    PRONOUN,
    ADJECTIVE,
    VERB,
    ADVERB,
    CONJUNCTION,
    PREPOSITION,
    INTERJECTION,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum WordOrder {
    SOV,
    OSV,
    OVS,
    VOS,
    SVO,
    VSO,
}

pub enum Noun {
    NounI,
    NounII,
}

#[derive(Debug)]
pub struct Phrase {
    // word_order: WordOrder,
    // verb: Vec<VerbI>,
    // objects: Vec<Object>,
}

pub struct Sentence {
    // phrases: Vec<Phrase>,
}

// lazy_static! {
//     static ref WordOrderDistribution: HashMap<&WordOrder, f32> = HashMap::from([
//         (&WordOrder::SOV, 0.63),
//         (&WordOrder::OSV, 0.21),
//         (&WordOrder::OVS, 0.6),
//         (&WordOrder::VOS, 0.5),
//         (&WordOrder::SVO, 0.4),
//         (&WordOrder::VSO, 0.1),
//     ]);
// };

// impl Phrase {
//     fn inflect(&self) {
//         let verb = &self.verb.conjugate(
//             Person::Third,
//             crate::grammar::latin::verb::Number::Singular,
//             Tense::Present,
//             Mood::Indicative,
//             Voice::Active,
//         );
//
//         for subject in self.subjects {
//             let subject = subject.decline(&Case::Nominative, &Number::Singular);
//         }
//         let obj = terra.decline(&Case::Accusative, &Number::Singular);
//     }
// }
//
// // fn construct_phrase(nouns: Vec<Box<dyn Noun>>) {
// //     for noun in nouns {
// //         let result = noun.decline(&Case::Nominative, &Number::Singular);
// //         println!("{}", result);
// //     }
// // }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_phrase() {
//         let stella = NounI {
//             nominative: "stella".to_string(),
//             genitive: "stellae".to_string(),
//             ..Default::default()
//         };
//         let terra = NounI {
//             nominative: "terra".to_string(),
//             genitive: "terrae".to_string(),
//             ..Default::default()
//         };
//         let illumino = VerbI {
//             present: "illumino".to_string(),
//             infinitive: "illuminare".to_string(),
//             perfect: "illuminavi".to_string(),
//             supine: "illuminatum".to_string(),
//             ..Default::default()
//         };
//         let phrase: Phrase = Phrase {
//             subjects: vec![stella],
//             verb: illumino,
//             objects: vec![terra],
//         };
//         assert_ne!(phrase, Phrase {});
//     }
// }
// // impl WordOrder {
// //     pub fn construct_phrase(&self, words: Vec<String>) -> String {
// //         match &self {
// //             WordOrder::SOV => "profugiunt statim ex urbe tribūnī plēbis.".to_string(),
// //             _ => "".to_string(),
// //         }
// //     }
// //     pub fn parse_word_order(&self) -> Vec<PoSD> {
// //         match &self {
// //             WordOrder::SOV => vec![PoSD::NOUN, PoSD::NOUN, PoSD::VERB],
// //             _ => panic!(),
// //         }
// //     }
//
// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //
// //     #[test]
// //     fn test_build_phrase() {
// //         let sov = WordOrder::SOV;
// //         let phrase = sov.construct_phrase();
// //         assert_eq!(phrase, "profugiunt statim ex urbe tribūnī plēbis.");
// //     }
// // }
