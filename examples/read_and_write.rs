use kserde::*;
fn main() {
    let source = std::fs::read_to_string("examples/hello.json").unwrap();
    let json = Thing::from_json(&source).unwrap();

    let generated_json = to_json_string(&json);
    println!("GENERATED JSON: {}", generated_json);
}
