use crate::request::HttpRequest;
use crate::request::ParseError;
use crate::response::HttpResponse;
use crate::status_code::StatusCode;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait HttpHandler {
    fn handle_request(&mut self, request: &HttpRequest) -> HttpResponse;

    fn handle_bad_request(&mut self, e: &ParseError) -> HttpResponse {
        HttpResponse::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self, mut handler: impl HttpHandler) {
        let listener = match TcpListener::bind(&self.address) {
            Ok(listener) => listener,
            Err(err) => panic!("{}", err.to_string()),
        };

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("## Got request");
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match HttpRequest::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("failed to establish a connection: {}", e),
            }
        }
    }
}
