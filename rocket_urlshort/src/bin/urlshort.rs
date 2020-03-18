//! The urlshort command line tool is an abstraction for the
//! database interaction layer (db module).

use rocket_urlshort::db;
use std::env;

fn help() {
    println!("subcommands:");
    println!("    new <short> <url>: add a new short url mapping");
    println!("    list: list all url mappings");
    println!("    remove <short>: removes the url mapping with key <short>");
    println!("    get <short>: displays the full url represented by <short>");
}

fn main() {
    // Get cmd line args
    let args: Vec<String> = env::args().collect();

    // Must have a subcommand
    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_url(&args[2..]),
        "list" => list_all_urls(&args[2..]),
        "remove" => remove_url(&args[2..]),
        "get" => get_url(&args[2..]),
        _ => help(),
    }
}

/// Adds a new (short => long) url mapping to the DB.
fn new_url(args: &[String]) {
    if args.len() < 2 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = db::establish_connection();
    match db::create_url_mapping(&conn, &args[0], &args[1]) {
        Ok(rows) => println!("Successfully inserted {} row", rows),
        Err(e) => eprintln!("Problem creating url mapping: {}", e),
    };
}

/// Prints all url mappings
fn list_all_urls(args: &[String]) {
    if args.len() > 0 {
        println!("show: does not take arguments");
        help();
        return;
    }

    let conn = db::establish_connection();
    match db::get_all_urls(&conn) {
        Ok(vector) => {
            println!("URLs\n----");
            for url in vector {
                println!("{} => {}", url.short, url.long);
            }
        }
        Err(e) => eprintln!("Problem getting values: {}", e),
    }
}

fn remove_url(args: &[String]) {
    if args.len() < 1 {
        println!("remove: missing <short>");
        help();
        return;
    }

    let conn = db::establish_connection();
    match db::delete_url_mapping(&conn, &args[0]) {
        Ok(rows) => println!("Removed {} rows", rows),
        Err(e) => eprintln!("Problem removing url: {}", e),
    }
}

fn get_url(args: &[String]) {
    if args.len() < 1 {
        println!("get: missing <short>");
        help();
        return;
    }

    let conn = db::establish_connection();
    let list = match db::get_url(&conn, &args[0]) {
        Ok(urls) => urls,
        Err(e) => {
            eprintln!("Problem getting urls: {}", e);
            return;
        }
    };
    for url in list {
        println!("{}", url);
    }
}
