use kjson::*;
fn main() {
    let source =
        std::fs::read_to_string("examples/hello.json")
            .unwrap();
    let json = from_str(&source);
    println!("JSON: {:#?}", json);
}
