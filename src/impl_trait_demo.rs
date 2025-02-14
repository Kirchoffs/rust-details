#[cfg(test)]
mod test {
    fn print_length_by_impl(s: impl AsRef<str>) {
        println!("{}", s.as_ref().len());
    }

    fn print_length_by_generics<T: AsRef<str>>(s: T) {
        println!("{}", s.as_ref().len());
    }

    fn apply_func_by_impl(f: impl Fn(i32) -> i32, val: i32) {
        println!("{}", f(val));
    }

    fn apply_func_by_generics_1<F: Fn(i32) -> i32>(f: F, val: i32) {
        println!("{}", f(val));
    }

    fn apply_func_by_generics_2<F>(f: F, val: i32)
    where
        F: Fn(i32) -> i32,
    {
        println!("{}", f(val));
    }

    fn apply_func_by_box_dyn(f: Box<dyn Fn(i32) -> i32>, val: i32) {
        println!("{}", f(val));
    }

    #[test]
    fn test() {
        let s = String::from("hello");
        print_length_by_impl(s.clone());
        print_length_by_generics(s.clone());

        let f = |x| x + 1;
        apply_func_by_impl(f, 1);
        apply_func_by_generics_1(f, 1);
        apply_func_by_generics_2(f, 1);
        apply_func_by_box_dyn(Box::new(f), 1);
    }
}
