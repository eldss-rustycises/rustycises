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

/// Removes the url mapping given by the key `short_url`.
pub fn delete_url_mapping(conn: &SqliteConnection, short_url: &str) -> QueryResult<usize> {
    use schema::urls::dsl::{short, urls};
    diesel::delete(urls)
        .filter(short.eq(short_url))
        .execute(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    // Running multiple tests in this fashion seems to fail due to concurrent
    // testing. I had a second test that also used the `create_url_mapping` function
    // and every time I ran `cargo test` one or the other would fail each time, and
    // it would be a different one to fail each time. For now I have added all the
    // functions into a single, happy-path test. This is not ideal, but I'm not yet
    // sure how to manage with the issue. There does not seem to be any other obvious
    // way to fix it within Diesel.
    #[test]
    fn add_and_retrieve_a_url() {
        let conn = establish_connection();
        conn.test_transaction::<_, Box<dyn Error>, _>(|| {
            // Add a url
            let one = create_url_mapping(
                &conn,
                "test",
                "https://doc.rust-lang.org/book/ch11-01-writing-tests.html",
            )?;
            // Number of rows inserted
            assert_eq!(1, one);

            // This line returns a reference to a Vector of Url objects
            let result = get_all_urls(&conn)?;
            assert_eq!(1, result.len());

            // Test the data returned
            let result = &result[0];
            let (res_short, res_long) = (&result.short, &result.long);
            // Tests
            assert_eq!(String::from("test"), res_short.to_string());
            assert_eq!(
                String::from("https://doc.rust-lang.org/book/ch11-01-writing-tests.html"),
                res_long.to_string(),
            );

            // Remove the data
            let rows_affected = delete_url_mapping(&conn, "test")?;
            assert_eq!(1, rows_affected);

            // Test no data returned
            let result = get_all_urls(&conn)?;
            assert_eq!(0, result.len());

            // Remove again
            let rows_affected = delete_url_mapping(&conn, "test")?;
            assert_eq!(0, rows_affected);

            Ok(())
        });
    }
}
