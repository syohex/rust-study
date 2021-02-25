mod handler;
mod method;
mod query_string;
mod request;
mod response;
mod server;
mod status_code;

use crate::handler::DefaultHandler;
use crate::server::Server;
use std::env;
use std::path::Path;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    let default_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("public");
    if let Some(path) = env::var("PUBLIC_PATH").ok() {
        server.run(DefaultHandler::new(&Path::new(&path)));
    } else {
        server.run(DefaultHandler::new(&default_path));
    }
}
