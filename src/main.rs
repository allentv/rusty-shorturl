#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod handlers;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![handlers::code, handlers::static_files, handlers::ping,],
        )
        .launch();
}
