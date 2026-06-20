use crate::schema::latin_words::{self};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::LatinPos"]
pub enum LatinPos {
    #[db_rename = "Verb"]
    Verb,
    #[db_rename = "Noun"]
    Noun,
    #[db_rename = "Pronoun"]
    Pronoun,
    #[db_rename = "Adjective"]
    Adjective,
    #[db_rename = "Adverb"]
    Adverb,
    #[db_rename = "Conjunction"]
    Conjunction,
    #[db_rename = "Preposition"]
    Preposition,
    #[db_rename = "Interjection"]
    Interjection,
}

#[derive(Debug, serde::Serialize, Clone, Queryable, Selectable)]
#[diesel(table_name = latin_words)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LatinWord {
    #[diesel(deserialize_as = Uuid)]
    pub id: Uuid,
    pub pos: LatinPos,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = latin_words)]
pub struct NewLatinWord {
    pub pos: LatinPos,
}

pub fn create_latin_word(
    cnx: &mut PgConnection,
    pos: LatinPos,
) -> Result<Uuid, diesel::result::Error> {
    use diesel::RunQueryDsl;
    let id: Uuid = diesel::insert_into(latin_words::table)
        .values(NewLatinWord { pos })
        .returning(latin_words::id)
        .get_result(cnx)?;
    Ok(id)
}
