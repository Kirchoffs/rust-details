#[cfg(test)]
mod test {
    use std::borrow::Cow;

    #[test]
    fn cow_demo() {
        let borrowed: Cow<str> = Cow::Borrowed("x");
        let cloned: Cow<str> = Cow::Owned(String::from("y"));
        
        let another_borrowed: Cow<str> = Cow::from("z");
        let another_cloned: Cow<str> = Cow::from(String::from("w"));

        println!("{}, {}, {}, {}", borrowed, cloned, another_borrowed, another_cloned);
    }
}
