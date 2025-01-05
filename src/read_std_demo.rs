#[cfg(test)]
mod test {
    use std::io;

    #[test]
    fn test() {
        println!("Enter input: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("You entered: {}", input);
    }
}
