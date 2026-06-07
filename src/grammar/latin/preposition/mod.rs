use crate::grammar::latin::noun::Case;

use crate::schema::latin_prepositions::{self};

use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Queryable, Selectable)]
#[diesel(table_name = latin_prepositions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(treat_none_as_default_value = false)]
pub struct Preposition {
    #[diesel(deserialize_as = Uuid)]
    pub id: Option<Uuid>,
    pub word: String,
    pub cases: Vec<Case>,
}

/// Owned version for deserialization
#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = latin_prepositions)]
pub struct NewPreposition {
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
