//! Latin First and Second Declension Adjectives

use crate::grammar::latin::noun::*;

impl<'a, N: Noun + 'a> Adjective<'a, N> {
    fn decline(&self) -> String {
        // TODO: reuse noun for declension
        "".to_string()
    }
}

#[cfg(test)]
mod test_adjective {

    #[test]
    fn test_adj_i_ii() {
        // TODO: implement basic instantiation logic
    }
}
