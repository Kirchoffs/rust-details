#[cfg(test)]
mod test {
    #[test]
    fn option_take_demo() {
        let mut x = Some(42);
        let y = x.take();
        
        assert_eq!(x, None);
        assert_eq!(y, Some(42));
    }

    #[test]
    fn option_reference_demo() {
        let x = Some(42);
        if let Some(v_ref) = &x {
            println!("Got a reference to {}", v_ref);
        }
    }
}
