use std::io::stdin;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let command = input.trim();

    Command::new(command).spawn()?;

    Ok(())
}
