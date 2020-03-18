#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use rocket_contrib::{databases::diesel, json::Json, result::Error};
use rocket_urlshort::db::{self, models::*};

/// Wrapper for responses.
#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Url>,
}

/// Creates db connection pool usable by handlers.
#[database("urldb")]
struct UrlDbConn(diesel::SqliteConnection);

/// Returns a list of all the url pairings in the database.
#[get("/")]
fn get_urls(conn: UrlDbConn) -> Result<Json<JsonApiResponse>, Error> {
    let response = match db::get_all_urls(&*conn) {
        Ok(vect) => JsonApiResponse { data: vect },
        Err(e) => return Err(e),
    };
    Ok(Json(response))
}

/// Adds url mappings from JSON into the database. Returns a list of the urls
/// that could not be created either due to DB error or url parsing error.
#[post("/add/json", format = "json", data = "<list>")]
fn add_urls(conn: UrlDbConn, list: Json<Vec<NewUrl>>) -> Json<Vec<NewUrl>> {
    let list = list.into_inner();
    let mut not_created = vec![];
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
        .mount("/", routes![get_urls, add_urls])
        .launch();
}
