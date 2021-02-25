use crate::method::Method;
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use crate::server::HttpHandler;
use crate::status_code::StatusCode;
use std::fs;
use std::path::Path;

pub struct DefaultHandler<'a> {
    public_path: &'a Path,
}

impl<'a> DefaultHandler<'a> {
    pub fn new(public_path: &'a Path) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = Path::new(self.public_path).join(file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                println!(
                    "## path={}, base={}",
                    path.display(),
                    self.public_path.display()
                );
                fs::read_to_string(path).ok()
            }
            Err(e) => {
                println!("## failed path(): {}", e.to_string());
                None
            }
        }
    }
}

impl HttpHandler for DefaultHandler<'_> {
    fn handle_request(&mut self, request: &HttpRequest) -> HttpResponse {
        println!("## Handle Request: {:?}", request);
        match request.method() {
            Method::GET => match request.path() {
                "/" => HttpResponse::new(StatusCode::Ok, self.read_file("index.html")),
                path => match self.read_file(path) {
                    Some(contents) => HttpResponse::new(StatusCode::Ok, Some(contents)),
                    None => HttpResponse::new(StatusCode::NotFound, None),
                },
            },
            _ => HttpResponse::new(StatusCode::NotFound, None),
        }
    }
}
