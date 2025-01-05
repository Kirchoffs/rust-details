#[cfg(test)]
mod test {
    #[test]
    fn simple_error_processing_demo() {
        use std::num::ParseIntError;

        #[derive(Debug)]
        enum MyError {
            Parse(ParseIntError),
            Other(String),
        }

        impl From<ParseIntError> for MyError {
            fn from(err: ParseIntError) -> MyError {
                MyError::Parse(err)
            }
        }

        fn parse_number(input: &str) -> Result<i32, MyError> {
            let number: i32 = input.parse()?;
            Ok(number)
        }

        match parse_number("42-X") {
            Ok(num) => println!("Parsed number: {}", num),
            Err(err) => println!("Error: {:?}", err),
        }
    }
}
