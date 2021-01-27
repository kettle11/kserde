use kjson::*;
fn main() {
    let source = std::fs::read_to_string(
        "/Users/ian/Workspace/kjson/tests/test_parsing/y_string_unicode_U+2064_invisible_plus.json",
    )
    .unwrap();
    let json = from_str(&source);
    println!("JSON: {:#?}", json);
}
