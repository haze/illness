#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

mod api;
mod types;
mod database;
mod util;

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate serde_json;
extern crate rocket_contrib;
extern crate rand;
use std::io;
use rocket::response::NamedFile;
use rocket_contrib::Json;
use types::{FileDetails};


#[get("/<filename>")]
fn retrieve(filename: String) -> Json<FileDetails> {
    Json(FileDetails {
        owner: types::User {
            id: 0,
            identity: String::from("test"),
        },
        id: String::new(),
        ipfs: String::new(),
        link: String::new(),
    })
}

#[get("/join")]
fn join_user() -> io::Result<NamedFile> {
    NamedFile::open("static/join.html")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![join_user, retrieve])
        .mount("/api", routes![api::join])
        .manage(database::init_pool())
        .launch();
}