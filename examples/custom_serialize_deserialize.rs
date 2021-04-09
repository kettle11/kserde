use kserde::*;

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

impl<'a> Deserialize<'a> for Person {
    fn deserialize<D: Deserializer<'a>>(deserializer: &mut D) -> Option<Self> {
        deserializer.begin_object().then(|| {})?;

        let mut name: Option<String> = None;
        let mut age = None;

        while let Some(p) = deserializer.has_property() {
            match &*p {
                "name" => name = Some(deserializer.string()?.to_string()),

                "age" => {
                    age = Some(deserializer.i64()?);
                }
                _ => {}
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
