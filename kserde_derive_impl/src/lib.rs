use kreflect::*;

pub fn kserde_serialize_impl(value: &Value) -> String {
    match value {
        Value::Struct(_struct) => {
            let mut lifetimes_and_generics = String::new();
            for lifetime in &_struct.generic_lifetimes {
                lifetimes_and_generics.push('\'');
                lifetimes_and_generics += lifetime;
                lifetimes_and_generics.push(',');
            }
            for generic in &_struct.generic_types {
                lifetimes_and_generics += generic;
                lifetimes_and_generics.push(',');
            }

            let mut properties = String::new();
            match &_struct.fields {
                Fields::Struct(fields) => {
                    for field in fields {
                        let name = field.name.as_ref().unwrap();
                        properties +=
                            &format!("    object.property(\"{}\", &self.{});\n", name, name);
                    }
                }
                Fields::Tuple(_) => todo!(),
                Fields::Unit => todo!(),
            }

            format!(
                r#"impl<{}> Serialize for {}<{}> {{
    fn serialize<E: Serializer>(&self, serializer: &mut E) {{
        let mut object = serializer.begin_object();
{}
        object.end_object();
    }}
}}"#,
                lifetimes_and_generics, _struct.name, lifetimes_and_generics, properties
            )
        }
        Value::Enum(_) => {
            todo!()
        }
    }
}

pub fn kserde_deserialize_impl(value: &Value) -> String {
    match value {
        Value::Struct(_struct) => {
            let mut lifetimes_and_generics = String::new();
            for lifetime in &_struct.generic_lifetimes {
                lifetimes_and_generics.push('\'');
                lifetimes_and_generics += lifetime;
                lifetimes_and_generics.push(',');
            }
            for generic in &_struct.generic_types {
                lifetimes_and_generics += generic;
                lifetimes_and_generics.push(',');
            }

            let mut deserialize_match = String::new();
            let mut properties_declaration = String::new();
            let mut property_assignment = String::new();

            match &_struct.fields {
                Fields::Struct(fields) => {
                    for field in fields {
                        let name = field.name.as_ref().unwrap();
                        let _type = field._type.as_string();
                        if _type.get(0..6).map_or(false, |s| s == "Option") {
                            properties_declaration +=
                                &format!("    let mut {}: {} = None;\n", name, _type);
                            property_assignment += &format!("    {},\n", name);
                            deserialize_match += &format!(
                                "                \"{}\" => {} = Some(<{}>::deserialize(deserializer)?),\n",
                                name, name, &_type[7.._type.len() - 1]
                            );
                        } else {
                            properties_declaration +=
                                &format!("    let mut {}: Option<{}> = None;\n", name, _type);
                            property_assignment += &format!("        {}: {}?,\n", name, name);
                            deserialize_match += &format!(
                                "                \"{}\" => {} = Some(<{}>::deserialize(deserializer)?),\n",
                                name, name, _type
                            );
                        }
                    }
                }
                Fields::Tuple(_) => todo!(),
                Fields::Unit => todo!(),
            }
            format!(
                r#"impl<'kserde, {}> Deserialize<'kserde> for {}<{}> {{
    fn deserialize<D: Deserializer<'kserde>>(deserializer: &mut D) -> Option<Self> {{
        deserializer.begin_object().then(|| {{}})?;
{}
        while let Some(p) = deserializer.has_property() {{
            match &*p {{
{}              _ => {{}}
            }}
        }}
        Some(Self {{
{}
        }})
    }}
}}"#,
                lifetimes_and_generics,
                _struct.name,
                lifetimes_and_generics,
                properties_declaration,
                deserialize_match,
                property_assignment
            )
        }
        Value::Enum(_) => {
            todo!()
        }
    }
}

#[test]
fn kersde_impl() {
    let value = Value::Struct(Struct {
        name: "Thing".into(),
        visibility: Visibility::Private,
        generic_lifetimes: Vec::new(),
        generic_types: Vec::new(),
        fields: Fields::Struct(vec![Field {
            name: Some("x".into()),
            _type: Type::Name(Path::new(&["f32".into()])),
            visibility: Visibility::Pub,
        }]),
    });

    println!("{}", kserde_deserialize_impl(&value));
}
