use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum ProgError {
    IOErr(std::io::Error),
    RegexError(regex::Error),
}

impl From<std::io::Error> for ProgError {
    fn from(err: std::io::Error) -> Self {
        ProgError::IOErr(err)
    }
}

impl From<regex::Error> for ProgError {
    fn from(err: regex::Error) -> Self {
        ProgError::RegexError(err)
    }
}

type ProgResult<T> = Result<T, ProgError>;

fn main() -> ProgResult<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("usage: {} pid", args[0]);
    }

    let pid = &args[1];
    match pid.parse::<i32>() {
        Ok(_) => (),
        Err(e) => panic!("can't parse {} as number: {:?}", pid, e),
    };

    let path = format!("/proc/{}/status", pid);
    let mut file = match File::open(path) {
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            panic!("PID {} is not found", pid);
        }
        Err(e) => panic!("io error: {:?}", e),
        Ok(f) => f,
    };
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let re = Regex::new(r"(?m)^State:\s+\S+\s+\(([^)]+)\)")?;
    let captures = re.captures(&content).unwrap();
    let status = captures.get(1).map_or("", |m| m.as_str());
    println!("Process {} status is {}", pid, status);

    Ok(())
}
