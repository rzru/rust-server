use super::server::Handler;
use super::http::{Request, Response, Method, StatusCode, Headers};
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Traversal attack attempted! Path: {}", file_path);
                    None
                }
            },
            Err(_) => None
        }
    }
}

impl Handler for WebsiteHandler {
    fn handler_request(&mut self, request: &Request) -> Response {
        let mut html_headers = Headers::new();
        html_headers.add("Content-Type", "text/html");

        let empty_headers = Headers::new();

        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html"), html_headers),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html"), html_headers),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents), empty_headers),
                    None => Response::new(StatusCode::NotFound, None, empty_headers),
                },
            }
            _ => Response::new(StatusCode::NotFound, None, empty_headers),
        }
    }
}