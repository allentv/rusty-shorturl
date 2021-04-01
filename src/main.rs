#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod handlers;

fn main() {
    let mut mnt = rocket::ignite();

    // Add root routes
    mnt = mnt.mount(
        "/",
        routes![handlers::code, handlers::static_files, handlers::ping,],
    );

    // Start the web server
    mnt.launch();
}
