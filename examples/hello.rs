use kjson::*;
fn main() {
    let source = std::fs::read_to_string("test.json").unwrap();
    let _json = parse_to_json(&source);
}
