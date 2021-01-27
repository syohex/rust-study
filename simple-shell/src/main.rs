use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() -> std::io::Result<()> {
    loop {
        print!("> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        let mut child = Command::new(command).args(args).spawn()?;

        child.wait()?;
    }
}
