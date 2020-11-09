use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Foo {
    name: String,
    age: i32,
}

impl FromStr for Foo {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name_age: Vec<&str> = s
            .trim_matches(|p| p == '[' || p == ']')
            .split(',')
            .collect();

        let name = name_age[0].to_string();
        let age = name_age[1].parse::<i32>()?;
        Ok(Foo {
            name: name,
            age: age,
        })
    }
}

fn main() {
    let f = "[yamada,84]".parse::<Foo>().unwrap();
    println!("f={:?}", f);
}
