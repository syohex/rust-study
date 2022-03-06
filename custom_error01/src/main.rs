#[derive(Debug)]
enum MyError {
    Message(&'static str),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "my error")
    }
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl From<&'static str> for MyError {
    fn from(name: &'static str) -> Self {
        MyError::Message(name)
    }
}

fn test() -> Result<(), &'static str> {
    Err("hoge")
}

fn main() -> Result<(), MyError> {
    test()?;
    Ok(())
}
