#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

#[database("shorty_db")]
pub struct ShortyDBConn(diesel::SqliteConnection);

use self::schema::shorty;
use rocket::response::NamedFile;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "shorty"]
pub struct ShortURL {
    pub id: i32,
    pub handle: String,
    pub full_url: String,
    pub created: String,
}

#[get("/<code>")]
fn code(_conn: ShortyDBConn, code: String) -> String {
    return format!("Received code: {}", code);
}

#[get("/ping")]
fn ping() -> String {
    return String::from("pong");
}

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    return NamedFile::open(Path::new("static/").join(file)).ok();
}

fn main() {
    let mut mnt = rocket::ignite().attach(ShortyDBConn::fairing());

    // Add root routes
    mnt = mnt.mount("/", routes![code, static_files, ping,]);

    // Start the web server
    mnt.launch();
}
