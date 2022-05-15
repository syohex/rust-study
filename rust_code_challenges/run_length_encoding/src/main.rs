mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut ret = String::new();
        if text.is_empty() {
            return ret;
        }

        let mut count = 1;
        let cs: Vec<char> = text.chars().collect();
        let len = cs.len();
        let mut prev = cs[0];

        for (i, c) in cs.into_iter().enumerate().skip(1) {
            if i == len - 1 {
                if c == prev {
                    count += 1;
                    ret.push_str(&count.to_string());
                    ret.push(c);
                } else {
                    ret.push_str(&count.to_string());
                    ret.push(prev);

                    ret.push('1');
                    ret.push(c);
                }
            } else if c != prev || count == 9 {
                ret.push_str(&count.to_string());
                ret.push(prev);

                prev = c;
                count = 1;
            } else {
                count += 1;
            }
        }

        ret
    }

    pub fn decode(text: &str) -> String {
        let mut ret = String::new();
        let mut count = 0;
        for c in text.chars() {
            if ('0'..='9').contains(&c) {
                count = c as u8 - b'0';
            } else {
                for _ in 0..count {
                    ret.push(c);
                }
                count = 0;
            }
        }

        ret
    }
}

fn main() {
    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    let encoded = run_length_encoding::encode(input);
    println!("## encoded={encoded}");
    let decoded = run_length_encoding::decode(&encoded);
    println!("## decoded={decoded}");
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
