#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

type Letter = Vec<Pulse>;

type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        use Pulse::*;

        let mut ret = vec![];
        for c in self.to_uppercase().chars() {
            let v = match c {
                'A' => vec![Short, Long],
                'B' => vec![Long, Short, Short, Short],
                'C' => vec![Long, Short, Long, Short],
                'D' => vec![Long, Short, Short],
                'E' => vec![Short],
                'F' => vec![Short, Short, Long, Short],
                'G' => vec![Long, Long, Short],
                'H' => vec![Short, Short, Short, Short],
                'I' => vec![Short, Short],
                'J' => vec![Short, Long, Long, Long],
                'K' => vec![Long, Short, Long],
                'L' => vec![Short, Long, Short, Short],
                'M' => vec![Long, Long],
                'N' => vec![Long, Short],
                'O' => vec![Long, Long, Long],
                'P' => vec![Short, Long, Long, Short],
                'Q' => vec![Long, Long, Short, Long],
                'R' => vec![Short, Long, Short],
                'S' => vec![Short, Short, Short],
                'T' => vec![Long],
                'U' => vec![Short, Short, Long],
                'V' => vec![Short, Short, Short, Long],
                'W' => vec![Short, Long, Long],
                'X' => vec![Long, Short, Short, Long],
                'Y' => vec![Long, Short, Long, Long],
                'Z' => vec![Long, Long, Short, Short],
                '0' => vec![Long, Long, Long, Long],
                '1' => vec![Short, Long, Long, Long, Long],
                '2' => vec![Short, Short, Long, Long, Long],
                '3' => vec![Short, Short, Short, Long, Long],
                '4' => vec![Short, Short, Short, Short, Long],
                '5' => vec![Short, Short, Short, Short, Short],
                '6' => vec![Long, Short, Short, Short, Short],
                '7' => vec![Long, Long, Short, Short, Short],
                '8' => vec![Long, Long, Long, Short, Short],
                '9' => vec![Long, Long, Long, Long, Short],
                _ => continue,
            };

            ret.push(v);
        }

        ret
    }
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

fn main() {
    let greeting = "Hello, world".to_string().to_morse_code();

    print_morse_code(&greeting);
}

#[test]
fn hello_world() {
    use Pulse::*;

    let expected = vec![
        vec![Short, Short, Short, Short],
        vec![Short],
        vec![Short, Long, Short, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Long, Long],
        vec![Short, Long, Long],
        vec![Long, Long, Long],
        vec![Short, Long, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Short, Short],
    ];

    let actual = "Hello, world".to_string().to_morse_code();
    assert_eq!(actual, expected);
}

#[test]
fn whole_alphabet() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();

    let a = alphabet.to_morse_code();
    let b = alphabet.to_uppercase().to_morse_code();
    assert_eq!(a, b);
}
