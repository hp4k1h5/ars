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

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "grammatical_case"))]
    pub struct GrammaticalCase;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "latin_pos"))]
    pub struct LatinPos;
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
    latin_lookup (id) {
        id -> Uuid,
        word -> Uuid,
        form -> Varchar,
        path -> Varchar,
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
    use super::sql_types::GrammaticalCase;

    latin_prepositions (id) {
        id -> Uuid,
        word -> Varchar,
        cases -> Array<Nullable<GrammaticalCase>>,
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

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LatinPos;

    latin_words (id) {
        id -> Uuid,
        pos -> LatinPos,
    }
}

diesel::joinable!(latin_adjectives -> latin_words (id));
diesel::joinable!(latin_lookup -> latin_words (word));
diesel::joinable!(latin_nouns -> latin_words (id));
diesel::joinable!(latin_prepositions -> latin_words (id));
diesel::joinable!(latin_verbs -> latin_words (id));

diesel::allow_tables_to_appear_in_same_query!(
    latin_adjectives,
    latin_lookup,
    latin_nouns,
    latin_prepositions,
    latin_verbs,
    latin_words,
);
