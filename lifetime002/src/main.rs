struct Suffix {
    suffix: String,
}

impl Suffix {
    fn stem<'a>(&self, word: &'a str) -> &'a str {
        if word.ends_with(&self.suffix) {
            let index = word.rfind(&self.suffix).expect("error");
            &word[0..index]
        } else {
            word
        }
    }
}

fn main() {
    let str = String::from(".rs");
    let s = Suffix { suffix: str };

    println!("## stem={}", s.stem("hello.rs"));
}
