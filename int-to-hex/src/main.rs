use std::io::prelude::*;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    println!("Input number >> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut num = input.parse::<i32>().unwrap();

    let mut v = Vec::<char>::new();
    loop {
        let m = num % 16;
        num = num / 16;

        let c = match m {
            0..=9 => std::char::from_digit(m as u32, 10).unwrap(),
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            15 => 'F',
            _ => panic!("never reach here"),
        };

        v.push(c);

        if num == 0 {
            break;
        }
    }

    v.push('x');
    v.push('0');

    v.reverse();
    let s: String = v.into_iter().collect();

    println!("");
    println!("Digimal {} => HEX = {}", input, s);

    Ok(())
}
