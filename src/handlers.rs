use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

#[get("/<code>")]
pub fn code(code: String) -> String {
    return format!("Received code: {}", code);
}

#[get("/ping")]
pub fn ping() -> String {
    return String::from("pong");
}

#[get("/static/<file..>")]
pub fn static_files(file: PathBuf) -> Option<NamedFile> {
    return NamedFile::open(Path::new("static/").join(file)).ok();
}
