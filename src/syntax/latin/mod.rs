use crate::grammar::latin::*;

pub mod word_order;

/// Parts of Speech
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
    Preposition,
    Conjunction,
}

pub struct Word {
    word: PoS,
    role: GrammaticalRole,
}

pub struct Text {
    title: String,
    author: String,
    words: Vec<String>,
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
        })
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
