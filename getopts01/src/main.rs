use std::env;
use getopts::Options;

static VERSION: &'static str = "1.2.3";

fn main() {
    let args : Vec<String> = env::args().map(|x| x.to_string()).collect();

    let mut opts = Options::new();
    opts.optopt("n", "number", "number of items", "NUM");
    opts.optflag("v", "version", "print version");
    opts.optflag("h", "help", "help");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) },
    };

    if matches.opt_present("h") {
        println!("version is {}", VERSION);
        return;
    }

    let num = matches.opt_get::<i32>("n");
    match num {
        Ok(n) => {
            if let Some(m) = n {
                println!("num option is {}", m);
            } else {
                println!("num option is not defined");
            }
        },
        Err(f) => {
            println!("parse error: {:?}", f);
        }
    }
}
