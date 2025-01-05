#[cfg(test)]
mod test {
    trait Animal {
        fn make_sound(&self);
    }
    
    impl dyn Animal {
        fn create(animal_type: &str) -> Box<dyn Animal> {
            match animal_type {
                "cat" => Box::new(Cat {}),
                "dog" => Box::new(Dog {}),
                _ => panic!("Unknown animal type")
            }
        }
    }
    
    struct Cat {}
    struct Dog {}
    
    impl Animal for Cat {
        fn make_sound(&self) {
            println!("Meow!");
        }
    }
    
    impl Animal for Dog {
        fn make_sound(&self) {
            println!("Woof!");
        }
    }

    #[test]
    fn test_impl_dyn_demo() {
        let cat: Box<dyn Animal> = <dyn Animal>::create("cat");
        cat.make_sound();
    
        let dog = <dyn Animal>::create("dog");
        dog.make_sound();
    }
}
