use kjson::*;

struct Person {
    name: String,
    age: i64,
}

impl Serialize for Person {
    fn serialize<E: Serializer>(&self, serializer: &mut E) {
        let mut object = serializer.begin_object();
        object.property("name", &self.name);
        object.property("age", &self.age);
        object.end_object();
    }
}

impl Deserialize for Person {
    fn deserialize<'a, D: Deserializer<'a>>(deserializer: &mut D) -> Option<Self> {
        let mut object = deserializer.begin_object()?;
        let mut name: Option<String> = None;
        let mut age = None;

        for _ in 0..2 {
            {
                let (property_name, mut value) = object.property()?;
                let property_name: &str = &property_name;
                match property_name {
                    "name" => {
                        name = Some(value.string()?.to_string());
                    }
                    "age" => {
                        age = Some(value.i64()?);
                    }
                    _ => {}
                }
            }
        }

        Some(Self {
            name: name?,
            age: age?,
        })
    }
}

fn main() {
    let person = Person {
        name: "Odysseus".to_string(),
        age: 43,
    };

    let mut serializer = JSONSerializer::new();
    person.serialize(&mut serializer);
    println!("{}", serializer.done());
}
