#[cfg(test)]
mod test {
    use std::ops::Range;

    #[test]
    fn std_ops_range_demo() {
        let range: Range<i32> = 0..5;

        let mut iterator = range.clone();
        while let Some(i) = iterator.next() {
            println!("{}", i);
        }

        let mut rev_iterator = range.clone();
        while let Some(i) = rev_iterator.next_back() {
            println!("{}", i);
        }
    }
}
