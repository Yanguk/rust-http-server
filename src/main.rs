#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod http;
mod server;
mod website_handler;


fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("경로 {}", public_path);

    let port = String::from("127.0.0.1:8080");

    let server = Server::new(port);

    server.run(WebsiteHandler::new(public_path));
}
