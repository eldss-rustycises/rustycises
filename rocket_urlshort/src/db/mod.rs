//! The db module will act as a database interaction layer that runs
//! queries directly on the database, through Diesel.

use diesel::{prelude::*, result::QueryResult, sqlite::SqliteConnection};
use std::env;
use std::error::Error;
use url::Url;

pub mod models;
pub mod schema;

/// Creates a connection to the DB. Panics if the DATABASE_URL
/// environment variable is not set or the database connection
/// fails.
pub fn establish_connection() -> SqliteConnection {
    // This env var must be set
    let key = "DATABASE_URL";
    let db_url = env::var(key).expect(&format!("Environment Variable {} not found", key));
    SqliteConnection::establish(&db_url)
        .unwrap_or_else(|err| panic!("Problem connecting to database: {}", err))
}

/// Creates a new url mapping.
///
/// `short_url` is a key to match with a full url (`long_url`).
/// Returns the number of rows inserted or an error.
///
/// # Errors
///
/// Will fail if the url is malformed or if there is a database
/// error.
pub fn create_url_mapping<'a>(
    conn: &SqliteConnection,
    short_url: &'a str,
    long_url: &'a str,
) -> Result<usize, Box<dyn Error>> {
    // Parse `long_url` to ensure valid url
    let _url = Url::parse(long_url)?;

    // Don't keep Url type 'cause sqlite doesn't have one
    let url_map = models::NewUrl {
        short: short_url,
        long: long_url,
    };

    // DB interaction
    let rows_created = diesel::insert_into(schema::urls::table)
        .values(&url_map)
        .execute(conn)?;

    Ok(rows_created)
}

/// Retrieves all Url mappings in the database.
pub fn get_all_urls(conn: &SqliteConnection) -> QueryResult<Vec<models::Url>> {
    schema::urls::table.load(conn)
}
