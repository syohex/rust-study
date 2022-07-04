use anyhow::Context;
use anyhow::Result;

fn main() -> Result<()> {
    let response = reqwest::blocking::get("https://syohex.org")?.text()?;
    let document = scraper::Html::parse_document(&response);

    let selector = scraper::Selector::parse("head > title").unwrap();
    let title = document
        .select(&selector)
        .map(|x| x.inner_html())
        .next()
        .context("title not found")?;

    println!("{title}");
    Ok(())
}
