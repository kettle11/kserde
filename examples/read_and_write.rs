use kjson::*;
fn main() {
    let source = std::fs::read_to_string("examples/hello.json").unwrap();
    let json = from_str(&source).unwrap();
    let generated_json = to_string(&json);
    println!("GENERATED JSON: {}", generated_json);
}