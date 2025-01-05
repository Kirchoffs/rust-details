#[cfg(test)]
mod test {
    #[test]
    fn as_ref_demo_hitotsu() {
        let s1 = String::from("Hello");
        let s2: &str = s1.as_ref();
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    #[test]
    fn as_ref_demo_futatsu() {
        struct IntegerWrapper {
            value: i32,
        }
    
        impl AsRef<i32> for IntegerWrapper {
            fn as_ref(&self) -> &i32 {
                &self.value
            }
        }
    
        fn print_value<T: AsRef<i32>>(val: T) {
            println!("val = {}", val.as_ref());
        }
    
        fn print_i32(val: &i32) {
            println!("val = {}", val);
        }

        let integer_wrapper = IntegerWrapper { value: 42 };
        print_i32(integer_wrapper.as_ref());
        print_value(integer_wrapper);
    }

    #[test]
    fn as_ref_demo_mittsu() {
        struct IntStrWrapper {
            val: i32,
            val_holder: String,
        }
    
        impl IntStrWrapper {
            fn new(val: i32) -> Self {
                Self { 
                    val,
                    val_holder: String::from(val.to_string()),
                }
            }
        }
    
        impl AsRef<str> for IntStrWrapper {
            fn as_ref(&self) -> &str {
                &self.val_holder
            }
        }
    
        fn print_str_ref<T: AsRef<str>>(t: T) {
            let t_ref = t.as_ref();
            println!("t_ref = {}", t_ref);
        }

        let t1 = String::from("Hello");
        print_str_ref(t1);

        let t2 = IntStrWrapper::new(42);
        print_str_ref(t2);
    }

    #[test]
    fn as_ref_demo_yottsu() {
        struct Rgb {
            red: u8,
            green: u8,
            blue: u8,
        }
        
        struct HexColor(String);
        
        impl AsRef<str> for Rgb {
            fn as_ref(&self) -> &str {
                Box::leak(Box::new(format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)))
            }
        }

        impl AsRef<str> for HexColor {
            fn as_ref(&self) -> &str {
                Box::leak(Box::new(format!("#{}", self.0)))
            }
        }

        fn print_color<C: AsRef<str>>(color: C) {
            println!("The color is: {}", color.as_ref());
        }

        let rgb = Rgb { red: 0, green: 255, blue: 0 };
        let hex_color = HexColor(String::from("FF0000"));

        print_color(rgb);
        print_color(hex_color);
    }
}
