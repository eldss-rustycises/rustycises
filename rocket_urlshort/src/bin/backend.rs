#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use diesel::result::Error;
use rocket::response::Redirect;
use rocket_contrib::{databases::diesel, json::Json};
use rocket_urlshort::db::{self, models::*};

/// Wrapper for responses.
#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Url>,
}

/// Creates db connection pool usable by handlers.
#[database("urldb")]
struct UrlDbConn(diesel::SqliteConnection);

/// Returns a list of all the url pairings in the database, or an error.
#[get("/")]
fn get_urls(conn: UrlDbConn) -> Result<Json<JsonApiResponse>, Error> {
    let response = match db::get_all_urls(&*conn) {
        Ok(vect) => JsonApiResponse { data: vect },
        Err(e) => return Err(e),
    };

    Ok(Json(response))
}

#[get("/goto/<short>")]
fn redirect_to_url(conn: UrlDbConn, short: String) -> Result<Redirect, Error> {
    let url_vect = db::get_url(&*conn, &short)?;
    if url_vect.len() == 0 {
        return Err(Error::NotFound);
    }
    Ok(Redirect::permanent(format!("{}", url_vect[0])))
}

/// Adds url mappings from JSON into the database. Returns a list of the urls
/// that could not be created either due to DB error or url parsing error.
#[post("/add/json", format = "json", data = "<list>")]
fn add_urls(conn: UrlDbConn, list: Json<Vec<NewUrl>>) -> Json<Vec<NewUrl>> {
    // Extract vector of NewUrl objects
    let list = list.into_inner();
    // Holds urls that were not created for some reason
    let mut not_created = vec![];
    // Add each mapping to DB
    for url in list {
        match db::create_url_mapping(&*conn, url.short, url.long) {
            Ok(_) => {}
            Err(_) => not_created.push(url),
        }
    }

    Json(not_created)
}

fn main() {
    rocket::ignite()
        .attach(UrlDbConn::fairing())
        .mount("/", routes![get_urls, add_urls, redirect_to_url])
        .launch();
}
