fn upper_last_char(s: &str) -> Option<char> {
    s.chars().last().map(|c| c.to_ascii_uppercase())
}

fn main() {
    println!(
        "upper_last_char('hello world')={:?}",
        upper_last_char("hello world")
    );
}

#[test]
fn test_upper_last_char() {
    assert_eq!(upper_last_char("hello"), Some('O'));
    assert_eq!(upper_last_char("HELLO"), Some('O'));
    assert_eq!(upper_last_char(""), None);
}
