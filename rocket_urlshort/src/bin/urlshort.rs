//! The urlshort command line tool is an abstraction for the
//! database interaction layer (db module).

use rocket_urlshort::db;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
}
