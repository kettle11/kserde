use kjson::*;
fn main() {
    let source =
        std::fs::read_to_string("/Users/ian/Workspace/kjson/tests/test_parsing/hello.json")
            .unwrap();
    let json = from_str(&source);
    println!("JSON: {:#?}", json);
}
