use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum IsbnError {
    Length,
    InvalidCharacter,
    IncorrectCheckDigit,
}

impl FromStr for Isbn {
    type Err = IsbnError; // TODO: replace with appropriate type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bs: Vec<u8> = s.bytes().filter(|b| *b != b'-').collect();
        if bs.len() != 13 {
            Err(IsbnError::Length)
        } else if !bs.iter().all(|b| (b'0'..=b'9').contains(b)) {
            Err(IsbnError::InvalidCharacter)
        } else {
            let digits: Vec<u8> = bs.into_iter().map(|b| b - b'0').collect();
            let check_digit = calculate_check_digit(&digits[0..12]);
            if check_digit != digits[12] {
                Err(IsbnError::IncorrectCheckDigit)
            } else {
                Ok(Self {
                    raw: s.to_string(),
                    digits,
                })
            }
        }
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let sum: u8 = digits
        .iter()
        .enumerate()
        .map(|(i, &n)| if i % 2 == 0 { n } else { 3u8 * n })
        .sum();
    let diff = 10 - (sum % 10);
    if diff <= 9 {
        diff as u8
    } else {
        0
    }
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();
    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);

    let check_digit = calculate_check_digit(&rust_in_action.digits[0..12]);
    println!(
        "CheckDigit: {check_digit}, expected: {}",
        rust_in_action.digits[12]
    );
}

#[test]
fn parse_test() {
    let ret: Result<Isbn, IsbnError> = "978-3-16-148410-0".parse();
    assert!(ret.is_ok());
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
