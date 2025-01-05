#[cfg(test)]
mod test {
    #[test]
    fn down_casting_demo_1() {
        use std::any::Any;

        fn print_if_string(value: &dyn Any) {
            if let Some(string) = value.downcast_ref::<String>() {
                println!("It's a String with value: {}", string);
            } else {
                println!("Not a String.");
            }
        }

        let my_string = String::from("Hello, Rust!");
        let my_number = 42;
    
        print_if_string(&my_string as &dyn Any); // It's a String with value: Hello, Rust!
        print_if_string(&my_number as &dyn Any); // Not a String.
    }
    
    #[test]
    fn down_casting_demo_2() {
        use std::any::Any;
        
        let my_value: Box<dyn Any> = Box::new(123);
    
        if let Ok(value) = my_value.downcast::<i32>() {
            println!("Successfully downcasted to i32: {}", value);
        } else {
            println!("Downcast failed.");
        }        
    }
}
