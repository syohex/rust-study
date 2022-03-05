use serde_json::json;

fn main() {
    let input_path = std::env::args().nth(1).expect("unspecified input file");
    let output_path = std::env::args().nth(2).expect("unspecified output file");

    let mut content = {
        let text = std::fs::read_to_string(&input_path).expect("failed to read");
        serde_json::from_str::<serde_json::Value>(&text).expect("failed to parse JSON")
    };

    content["hobbies"]
        .as_array_mut()
        .unwrap()
        .push(json!("guitar"));

    std::fs::write(output_path, serde_json::to_string_pretty(&content).unwrap())
        .expect("failed to write");
}
