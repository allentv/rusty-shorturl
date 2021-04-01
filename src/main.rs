#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket_contrib::databases::diesel;

#[database("shorty_db")]
pub struct ShortyDBConn(diesel::SqliteConnection);

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

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
