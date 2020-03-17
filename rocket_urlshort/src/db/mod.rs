//! The db module will act as a database interaction layer that runs
//! queries directly on the database, through Diesel.

use diesel::{prelude::*, sqlite::SqliteConnection};
use std::env;

pub mod models;
pub mod schema;

/// Creates a connection to the DB.
pub fn establish_connection() -> SqliteConnection {
    // This env var must be set
    let key = "DATABASE_URL";
    let db_url = env::var(key).expect(&format!("Environment Variable {} not found", key));
    SqliteConnection::establish(&db_url)
        .unwrap_or_else(|err| panic!("Problem connecting to database: {}", err))
}
