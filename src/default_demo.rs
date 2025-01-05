#[cfg(test)]
mod test {
    #[derive(Default)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    #[test]
    fn default_value_demo() {
        let default_point = Point::default();
        println!("Default Point: x = {}, y = {}", default_point.x, default_point.y);
    
        let custom_point = Point { x: 1, ..Default::default() };
        println!("Custom Point: x = {}, y = {}", custom_point.x, custom_point.y);
    }

    struct CustomPoint {
        x: i32,
        y: i32,
    }
    
    impl Default for CustomPoint {
        fn default() -> Self {
            CustomPoint { x: 42, y: 500 }
        }
    }
    
    #[test]
    fn implement_default_trait_demo() {
        let custom_default_point = CustomPoint::default();
        println!("Custom Default Point: x = {}, y = {}", custom_default_point.x, custom_default_point.y);
    }
    

}