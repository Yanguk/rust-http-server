#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;


fn main() {
    let port = String::from("127.0.0.1:8080");
    let server = Server::new(port);

    server.run(WebsiteHandler);
}
