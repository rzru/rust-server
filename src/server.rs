use std::net::TcpListener;
use std::io::{Read, Write};
use std::convert::TryFrom;
use crate::http::{Request, Response, StatusCode};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Server is listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                             let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>yay</h1>"),
                                    )
                                }
                                Err => {
                                    Response::new(
                                        StatusCode::BadRequest,
                                        None,
                                    )
                                },
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
