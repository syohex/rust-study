extern crate hyper;

use std::env;

use hyper::{body::HttpBody as _, Client};
use hyper_tls::HttpsConnector;
use tokio::io::{self, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client url");
            return Ok(());
        }
    };

    let url = url.parse::<hyper::Uri>().unwrap();

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    println!("\n\nDone!!");

    Ok(())
}
