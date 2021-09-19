use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;
use crate::http::{Request, Response, StatusCode, ParseError};

pub trait Handler {
    fn handler_request(&mut self, request: &Request) -> Response;

    fn handler_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Server is listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handler_request(&request),
                                Err(e) => handler.handler_bad_request(&e)
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e)
                            }
                        }
                        Err(e) => println!("Failed to accept a message: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish connection: {}", e)
            }
        }
    }
}
