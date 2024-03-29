use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler  {
    fn handle_request(&self, request: &Request) -> Response;

    fn handle_bad_request(&self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self, handler:impl Handler) {
        println!("listening on {}", &self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut buffer: [u8; 1024] = [0; 1024];

                    let response = match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            }
                        }
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                            Response::new(
                                StatusCode::BadRequest,
                                None
                            )
                        },
                    };

                    if let Err(e) = response.send(&mut stream) {
                        println!("Failed to send response: {}", e);
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
