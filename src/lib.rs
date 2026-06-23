use diesel::{Connection, PgConnection};
use std::env;

pub mod api;
pub mod grammar;
pub mod schema;
pub mod syntax;

pub fn establish_cnx() -> PgConnection {
    let env_type = env::var("ARS_ENV").unwrap_or_else(|_| "dev".to_string());
    let env_file = format!(".env.{}", env_type);
    match dotenvy::from_filename(&env_file) {
        Ok(_) => println!("Loaded {} successfully", env_file),
        Err(_) => println!("No {} file found, relying on system environment", env_file),
    }

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
