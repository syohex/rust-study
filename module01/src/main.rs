mod foo;

use crate::foo::{func1, func2};

fn main() {
    println!("{} {}", func1(42), func2(42));
}
