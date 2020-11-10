use std::result::*;

fn main() -> Result<(), reqwest::Error> {
    let resp = reqwest::blocking::get("https://syohex.org/")?;
    let body = resp.text();

    match body {
        Ok(text) => println!("{}", text),
        _ => println!("error"),
    }
    Ok(())
}
