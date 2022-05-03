use std::fmt::Display;

fn print_any_text_type<T: AsRef<str> + Display + ?Sized>(text: &T) {
    let s = text.as_ref();
    print!("{}", s);
}

fn main() {
    let str = "hello world\n";
    print_any_text_type(str);

    let string = "hello String\n".to_string();
    print_any_text_type(&string);
}
