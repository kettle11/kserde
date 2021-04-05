use kjson::*;

struct Person {
    name: &'static str,
    age: i64,
}

impl Serialize for Person {
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        self.name.serialize(serializer);
        self.age.serialize(serializer);
    }
}

fn main() {
    let person = Person {
        name: "Odysseus",
        age: 43,
    };

    let mut serializer = JSONSerializer::new();
    person.serialize(&mut serializer);
}
