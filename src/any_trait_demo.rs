#[cfg(test)]
mod test {
    use std::any::Any;
    
    #[test]
    fn any_trait_down_cast_demo() {
        fn print_if_string_1(value: &dyn Any) {
            if let Some(string) = value.downcast_ref::<String>() {
                println!("It's a String with value: {}", string);
            } else {
                println!("Not a String.");
            }
        }

        fn print_if_string_2<T: Any>(value: &T) {
            let any_value = value as &dyn Any;
            if let Some(string) = any_value.downcast_ref::<String>() {
                println!("It's a string with value: {}", string);
            } else {
                println!("Not a string");
            }
        }

        let my_string = String::from("Hello, Rust!");
        let my_number = 42;
    
        print_if_string_1(&my_string as &dyn Any); // It's a String with value: Hello, Rust!
        print_if_string_1(&my_number as &dyn Any); // Not a String.

        print_if_string_2(&my_string);             // It's a String with value: Hello, Rust!
        print_if_string_2(&my_number);             // Not a String.
    }    
}
