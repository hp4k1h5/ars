// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "adj_declension"))]
    pub struct AdjDeclension;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "conjugation"))]
    pub struct Conjugation;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "declension"))]
    pub struct Declension;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "gender"))]
    pub struct Gender;
}

diesel::define_sql_function! {
    /// Maps the PostgreSQL unaccent extension.
    fn unaccent(input: diesel::sql_types::Text) -> diesel::sql_types::Text;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AdjDeclension;

    latin_adjectives (id) {
        id -> Uuid,
        declension -> AdjDeclension,
        f -> Varchar,
        m -> Varchar,
        n -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Declension;
    use super::sql_types::Gender;

    latin_nouns (id) {
        id -> Uuid,
        declension -> Declension,
        nominative -> Varchar,
        genitive -> Varchar,
        gender -> Gender,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Conjugation;

    latin_verbs (id) {
        id -> Uuid,
        conjugation -> Conjugation,
        present -> Varchar,
        infinitive -> Varchar,
        perfect -> Varchar,
        supine -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(latin_adjectives, latin_nouns, latin_verbs,);
