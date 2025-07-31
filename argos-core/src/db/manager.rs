use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;
use dotenvy::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar definida");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error conectando a {}", database_url))
}
