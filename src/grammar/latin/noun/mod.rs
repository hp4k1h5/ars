use crate::grammar::latin::Number;
use crate::schema::latin_nouns::{self};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::Deserialize;
use uuid::Uuid;

pub mod i;
pub mod ii;
pub mod iii;
pub mod iv;
pub mod v;

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, DbEnum, utoipa::ToSchema,
)]
#[ExistingTypePath = "crate::schema::sql_types::Declension"]
pub enum Declension {
    #[db_rename = "I"]
    I,
    #[db_rename = "II"]
    II,
    #[db_rename = "III"]
    III,
    #[db_rename = "IV"]
    IV,
    #[db_rename = "V"]
    V,
}

#[derive(
    strum::EnumIter,
    Debug,
    PartialEq,
    Clone,
    Copy,
    DbEnum,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[ExistingTypePath = "crate::schema::sql_types::GrammaticalCase"]
pub enum Case {
    #[db_rename = "Nominative"]
    Nominative,
    #[db_rename = "Genitive"]
    Genitive,
    #[db_rename = "Dative"]
    Dative,
    #[db_rename = "Accusative"]
    Accusative,
    #[db_rename = "Ablative"]
    Ablative,
    #[db_rename = "Vocative"]
    Vocative,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, DbEnum, utoipa::ToSchema)]
#[ExistingTypePath = "crate::schema::sql_types::Gender"]
pub enum Gender {
    #[db_rename = "Feminine"]
    Feminine,
    #[db_rename = "Masculine"]
    Masculine,
    #[db_rename = "Neuter"]
    Neuter,
}

impl<'de> Deserialize<'de> for Gender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "f" | "feminine" => Ok(Gender::Feminine),
            "m" | "masculine" => Ok(Gender::Masculine),
            "n" | "neuter" => Ok(Gender::Neuter),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown gender `{}`; expected f/Feminine, m/Masculine, or n/Neuter",
                s
            ))),
        }
    }
}

/// Nominal declension, nominative and genitive forms
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    PartialEq,
    utoipa::ToSchema,
)]
#[diesel(table_name = latin_nouns)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Noun {
    #[diesel(deserialize_as = Uuid)]
    #[serde(default)]
    pub id: Option<Uuid>,
    pub declension: Declension,
    pub nominative: String,
    pub genitive: String,
    pub gender: Gender,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NounInstance<'a> {
    pub noun: &'a Noun,
    pub case: Case,
    pub number: Number,
}

impl NounInstance<'_> {
    pub fn decline(&self) -> String {
        match self.noun.declension {
            Declension::I => self.decline_i(),
            Declension::II => self.decline_ii(),
            Declension::III => self.decline_iii(),
            Declension::IV => self.decline_iv(),
            Declension::V => self.decline_v(),
        }
    }
}
