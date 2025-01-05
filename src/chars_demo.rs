#[cfg(test)]
mod test {
    use std::str::Chars;

    #[test]
    fn chars_demo() {
        let str = "Hello, Rust!";

        let itr: Chars = str.chars();
        for ch in itr {
            println!("{}", ch);
        }
    }

    #[test]
    fn peekable_chars_demo() {
        let str = "Hello, Rust!";

        let mut itr = str.chars().peekable();
        assert_eq!(itr.peek(), Some(&'H'));
        assert_eq!(itr.next(), Some('H'));
    }
}
