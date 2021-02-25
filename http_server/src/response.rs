use crate::status_code::StatusCode;
use std::io::{Result as IoResult, Write};

pub struct HttpResponse {
    status_code: StatusCode,
    body: Option<String>,
}

impl HttpResponse {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        println!("## Write response: {}", body);

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
