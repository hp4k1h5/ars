use crate::grammar::latin::{Number, noun::*};

pub mod i_ii;

#[derive(Debug)]
pub struct Adjective {
    pub f: String,
    pub m: String,
    pub n: String,
}

pub struct AdjectiveInstance {
    adjective: Adjective,
    case: Case,
    number: Number,
}

// impl AdjectiveInstance {
//     pub(super) fn inflect(&self) -> String {
//         let ni = NounInstance {
//             noun: Noun {
//                 declension:
//             },
//             case: self.case,
//             number: self.number,
//         };
//     }
// }
//

