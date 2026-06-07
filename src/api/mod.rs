pub mod app;
pub mod latin;
pub mod middleware;

diesel::define_sql_function! {
    /// Maps the PostgreSQL unaccent extension.
    fn unaccent(input: diesel::sql_types::Text) -> diesel::sql_types::Text;
}
