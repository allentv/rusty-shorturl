// This is taken from the offical hello world example for actix
use actix_web::{middleware, web, App, HttpServer};

mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .route("/index.html", web::get().to(handlers::index))
            .route("/", web::get().to(handlers::index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
