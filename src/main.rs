#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;

#[get("/<code>")]
fn code(code: String) -> String {
    return format!("Received code: {}", code);
}

fn main() {
    rocket::ignite().mount("/", routes![code]).launch();
}
