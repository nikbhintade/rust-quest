use serde_json::Value;
use std::env;

fn get_value<'a>(key: &str, json: &'a Value) -> Option<&'a Value> {
    json.get(key)
}

fn read_command_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("key argument is missing");
        return None;
    }
    // Here args vector will be dropped so we need to clone the key and return it
    Some(args[1].clone())
}

fn read_file(file_path: &str) -> Value {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to read file");

    serde_json::from_str(&file_content).expect("Failed to parse JSON")
}

fn main() {
    let key = read_command_args().expect("msg");
    let json = read_file("./crates/json_reader/input_json_reader.json");

    let value = get_value(&key, &json).expect("Key not found in JSON");
    println!("Value: {}", value);
}
