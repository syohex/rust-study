use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let p = Person {
        name: String::from("Yamada Taro"),
        age: 75,
    };

    let json = serde_json::to_string(&p).unwrap();
    println!("Person dump as JSON: {}", json);
}
