use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb(u8, u8, u8);

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.0
    }

    fn g(&self) -> u8 {
        self.1
    }

    fn b(&self) -> u8 {
        self.2
    }
}

impl FromStr for Rgb {
    type Err = i32;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 {
            panic!("invalid length: {s}")
        }

        let bs: Vec<u8> = s.bytes().collect();
        if bs[0] != b'#' {
            panic!("first character must be hash mark: {}", bs[0] as char);
        }

        let hexes: Vec<u8> = bs
            .into_iter()
            .skip(1)
            .map(|b| {
                if b >= b'0' && b <= b'9' {
                    b - b'0'
                } else if b >= b'a' && b <= b'f' {
                    b - b'a' + 10
                } else if b >= b'A' && b <= b'F' {
                    b - b'A' + 10
                } else {
                    panic!("invalid character: {}", b as char);
                }
            })
            .collect();

        let r = hexes[0] * 16 + hexes[1];
        let g = hexes[2] * 16 + hexes[3];
        let b = hexes[4] * 16 + hexes[5];

        Ok(Self(r, g, b))
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    let hex = "#FFBB84".to_string();
    let color: Rgb = hex.parse().unwrap();
    println!("color={color}");
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}
