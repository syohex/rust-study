use chrono::{Datelike, Local};

fn is_leap_year(year: i32) -> bool {
    if year % 400 == 0 {
        return true;
    }
    if year % 4 == 0 && year % 100 == 0 {
        return false;
    }

    year % 4 == 0
}

#[test]
fn test_leap_year() {
    assert!(!is_leap_year(1));
    assert!(is_leap_year(4));
    assert!(!is_leap_year(100));
    assert!(is_leap_year(400));
    assert!(is_leap_year(2000));
}

fn main() {
    let now = Local::now();
    println!("Today is {}", now);
    println!("This year is leap year:{}", is_leap_year(now.year()));
}
