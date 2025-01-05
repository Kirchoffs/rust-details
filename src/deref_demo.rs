#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::fmt::Debug;
 
    struct CustomizedBox<T: Debug>(T);

    impl<T: Debug> CustomizedBox<T> {
        fn new(v: T) -> Self {
            CustomizedBox(v)
        }
    }

    impl<T: Debug> Deref for CustomizedBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Debug> Drop for CustomizedBox<T> {
        fn drop(&mut self) {
            println!("{:?} is dropping", self.0);
        }
    }

    fn print_string(s: &String) {
        println!("{}", *s);
    }

    #[test]
    fn defer_demo() {
        let i = 1;
        let i_r = &i;
        let i_d = *i_r;
        assert_eq!(i_d, 1);

        let built_in_box = Box::new(1);
        let box_value = *built_in_box;
        assert_eq!(box_value, 1);

        let customized_box = CustomizedBox::<i32>::new(1);
        // let box_value = *(customized_box.deref());
        let box_value = *customized_box;
        assert_eq!(box_value, 1);
        
        {
            let customized_box_string = CustomizedBox::<String>::new("hello".to_string());
            print_string(customized_box_string.deref());
            print_string(&customized_box_string);
        }
    }
}
