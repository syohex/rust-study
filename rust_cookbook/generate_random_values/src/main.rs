use rand::distributions::{Alphanumeric, Distribution, Standard};
use rand::Rng;

fn generate_random_numbers() {
    let mut generator = rand::thread_rng();

    let n1: u8 = generator.gen();
    let n2: u32 = generator.gen();

    println!("u8 random={n1}, u32 random={n2}");

    let f1 = generator.gen::<f32>();
    let f2 = generator.gen::<f64>();
    println!("f32 random={f1}, f64 random={f2}");
}

fn generate_random_number_range() {
    let mut gen = rand::thread_rng();
    for i in 0..10 {
        println!("{i}: {}", gen.gen_range(0.0..50.0));
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn generate_point() {
    let mut gen = rand::thread_rng();

    println!("generate Point");
    for i in 0..10 {
        let p: Point = gen.gen();
        println!("# {i} x={}, y={}", p.x, p.y);
    }
}

fn generate_password() {
    println!("[generate password]");

    let random_str: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    println!("generate_password={random_str}");
}

fn main() {
    generate_random_numbers();
    generate_random_number_range();
    generate_point();
    generate_password();
}
