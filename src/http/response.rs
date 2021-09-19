use super::StatusCode;
use std::io::{Write, Result as IoResult};
use crate::http::headers::Headers;

pub struct Response<'a> {
    status_code: StatusCode,
    body: Option<String>,
    headers: Headers<'a>,
}

impl<'a> Response<'a> {
    pub fn new(status_code: StatusCode, body: Option<String>, headers: Headers<'a>) -> Self {
        Self { status_code, body, headers }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n{}\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            self.headers,
            body,
        )
    }
}
