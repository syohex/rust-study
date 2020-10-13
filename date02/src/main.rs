use chrono::{DateTime, Datelike, Duration, Local, Weekday};

fn sunday_of_one_years_ago(date: DateTime<Local>) -> DateTime<Local> {
    let ago = date.checked_sub_signed(Duration::days(365)).unwrap();
    let diff = match ago.weekday() {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 0,
    };

    ago.checked_sub_signed(Duration::days(diff)).unwrap()
}

fn main() {
    let now = Local::now();
    let ago = sunday_of_one_years_ago(now);
    println!("ago: {}", ago);
}
