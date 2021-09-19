use super::StatusCode;
use std::io::{Write, Result as IoResult};
use crate::http::headers::Headers;
use std::collections::HashMap;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        let mut headers_hashmap = HashMap::new();
        headers_hashmap.insert("Content-Type", "text/html");

        let headers = Headers::new(headers_hashmap);

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n{}\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            headers,
            body,
        )
    }
}
