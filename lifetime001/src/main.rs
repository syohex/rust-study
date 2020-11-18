use rand::Rng;

fn string_ref<'a>(a: &'a str, b: &'a str) -> &'a str {
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen();
    if num % 2 == 0 {
        a
    } else {
        b
    }
}
fn main() {
    println!("ret = {}", string_ref("hello", "world"));
}
