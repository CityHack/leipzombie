#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate glob;
#[macro_use]
extern crate lazy_static;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate regex;

mod data;

use std::io;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::{JSON, Value};

use data::collect_data;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/query/<query>")]
fn get(query: String) -> Option<JSON<HashMap<String, Vec<String>>>> {
    let data = collect_data(&query);
    Some(JSON(data))
}

#[error(404)]
fn not_found() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, get, files])
        .catch(errors![not_found])
        .launch();
}
