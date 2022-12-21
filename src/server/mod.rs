use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("listening on {}", &self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut buffer: [u8; 1024] = [0; 1024];

                    let response = match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let request = Request::try_from(&buffer);
                            dbg!(request);

                            Response::new(
                                StatusCode::Ok,
                                Some("<h1>Hello World</h1>".to_string()),
                            )
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
