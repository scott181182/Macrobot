#![warn(clippy::all)]

#[macro_use]
extern crate log;
use actix_files::Files;
use actix_web::{ get, web, App, HttpServer, Responder };

const PORT: u16 = 8073;

mod websocket;
use websocket::init_ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(init_ws)
            .service(Files::new("/", "./website/public").index_file("index.html"))
        })
        .bind(format!("127.0.0.1:{}", PORT))?
        .run()
        .await
}

#[get("/")]
async fn basic_response(
    web::Path(()) : web::Path<()>
) -> impl Responder {
    "Hello, World!"
}