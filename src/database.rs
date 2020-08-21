#[macro_use]
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::sqlite::*;

pub fn test(){
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}