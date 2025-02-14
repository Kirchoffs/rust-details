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

        fn make_unknown_sound(&self) {
            println!("Unknown animal sound");
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
        cat.make_unknown_sound();
    
        let dog = <dyn Animal>::create("dog");
        dog.make_sound();
        dog.make_unknown_sound();

        let second_cat = Cat {};
        second_cat.make_sound();
        // This will not compile:
        // second_cat.make_unknown_sound();

        let third_cat = Box::new(Cat {});
        third_cat.make_sound();
        // This will not compile:
        // third_cat.make_unknown_sound();

        let fourth_cat: Box<dyn Animal> = Box::new(Cat {});
        fourth_cat.make_sound();
        fourth_cat.make_unknown_sound();

        // This will not compile:
        // <dyn Animal>::make_unknown_sound();
    }
}
