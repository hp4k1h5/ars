use crate::grammar::latin::noun::Case;
use crate::schema::latin_prepositions::{self};
use crate::schema::sql_types::GrammaticalCase;

use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

impl diesel::deserialize::FromSql<diesel::sql_types::Nullable<GrammaticalCase>, diesel::pg::Pg>
    for Case
{
    fn from_sql(
        bytes: diesel::pg::PgValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        <Case as diesel::deserialize::FromSql<GrammaticalCase, diesel::pg::Pg>>::from_sql(bytes)
    }

    fn from_nullable_sql(bytes: Option<diesel::pg::PgValue<'_>>) -> diesel::deserialize::Result<Self> {
        match bytes {
            Some(bytes) => {
                <Case as diesel::deserialize::FromSql<GrammaticalCase, diesel::pg::Pg>>::from_sql(bytes)
            }
            None => Err("Unexpected null for Case".into()),
        }
    }
}

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
