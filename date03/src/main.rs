use chrono::{Datelike, Local, NaiveDate};

fn main() {
    let now = Local::now();
    let today = NaiveDate::from_ymd(now.year(), now.month(), now.day());
    let first_day = NaiveDate::from_ymd(now.year(), 1, 1);

    let diff = today - first_day;
    println!("Today is {}th day in {}", diff.num_days(), now.year());
}
