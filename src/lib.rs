use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::env;

pub mod api;
pub mod grammar;
pub mod schema;
pub mod syntax;

pub fn establish_cnx() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
