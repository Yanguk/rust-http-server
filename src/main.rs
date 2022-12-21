#![allow(dead_code)]

mod http;
mod server;

use http::Method;
use server::Server;

fn main() {
    let port = String::from("127.0.0.1:8080");
    let server = Server::new(port);

    server.run();
}
