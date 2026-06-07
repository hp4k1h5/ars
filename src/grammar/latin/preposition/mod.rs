use crate::grammar::latin::noun::Case;

pub struct Preposition {
    pub id: Option<String>,
    pub word: String,
    pub cases: Vec<Case>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verb_i() {
        let prep = Preposition {
            id: None,
            word: "ad".to_string(),
            cases: vec![Case::Accusative],
        };

        assert_eq!(prep.word, "ad")
    }
}
