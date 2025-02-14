#[cfg(test)]
mod test {
    use serde::{Serialize, Serializer};

    #[test]
    fn serde_json_serialize_and_deserialize_demo() {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        let person = Person {
            name: "Alice".to_string(),
            age: 42,
        };

        let serialized_person = serde_json::to_string(&person).unwrap();
        println!("serialized_person: {}", serialized_person);

        let deserialized_person: Person = serde_json::from_str(&serialized_person).unwrap();
        println!("deserialized_person: {:?}", deserialized_person);
    }

    #[test]
    fn serde_json_deserialize_demo() {
        #[derive(serde::Deserialize, Debug)]
        struct Person<'a> {
            name: &'a str,
            age: u8,
        }

        let json_data = r#"{"name": "Alice", "age": 42}"#;
        let deserialized_person: Person = serde_json::from_str(json_data).unwrap();
        println!("deserialized_person: {:?}", deserialized_person);
    }

    #[test]
    fn serde_json_deserialize_owned_demo() {
        #[derive(serde::Deserialize, Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        fn parse_json<T: serde::de::DeserializeOwned>(json_data: &str) -> T {
            serde_json::from_str(json_data).unwrap()
        }

        let json_data = r#"{"name": "Alice", "age": 42}"#;
        let deserialized_person: Person = parse_json(json_data);
        println!("deserialized_person: {:?}", deserialized_person);
    }
}
