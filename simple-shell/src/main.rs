use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
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

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let dir = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(dir) {
                    eprintln!("{}", e);
                } else {
                    println!("change directory to {:?}", dir);
                }
            }
            command => {
                let mut child = Command::new(command).args(args).spawn()?;
                child.wait()?;
            }
        }
    }
}
