#[cfg(test)]
mod test {
    #[test]
    fn mut_self_use_case_builder_demo() {
        #[derive(Debug)]
        struct Student {
            id: u32,
            name: String,
            age: u8,
        }

        struct StudentBuilder {
            id: Option<u32>,
            name: Option<String>,
            age: Option<u8>,
        }

        impl StudentBuilder {
            fn new() -> Self {
                Self {
                    id: None,
                    name: None,
                    age: None,
                }
            }

            fn set_id(mut self, id: u32) -> Self {
                self.id = Some(id);
                self
            }

            fn set_name(mut self, name: &str) -> Self {
                self.name = Some(name.into());
                self
            }

            fn set_age(mut self, age: u8) -> Self {
                self.age = Some(age);
                self
            }

            fn build(self) -> Option<Student> {
                Some(Student {
                    id: self.id?,
                    name: self.name?,
                    age: self.age?,
                })
            }
        }

        let student = StudentBuilder::new()
            .set_id(42)
            .set_name("Leo".into())
            .set_age(25)
            .build();

        println!("{:?}", student);
    }
}