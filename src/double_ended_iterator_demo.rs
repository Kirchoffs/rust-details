#[cfg(test)]
mod test {
    struct MyDoubleEndedIterator {
        data: Vec<i32>,
        front: usize,
        back: usize,
    }
    
    impl MyDoubleEndedIterator {
        fn new(data: Vec<i32>) -> Self {
            let len = data.len();
            Self {
                data,
                front: 0,
                back: len,
            }
        }
    }
    
    // Because of `trait DoubleEndedIterator: Iterator`,
    // if you implement `DoubleEndedIterator`, you must also implement `Iterator`.
    impl DoubleEndedIterator for MyDoubleEndedIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.front < self.back {
                self.back -= 1;
                Some(self.data[self.back])
            } else {
                None
            }
        }
    }

    impl Iterator for MyDoubleEndedIterator {
        type Item = i32;
    
        fn next(&mut self) -> Option<Self::Item> {
            if self.front < self.back {
                let item = self.data[self.front];
                self.front += 1;
                Some(item)
            } else {
                None
            }
        }
    }
    
    #[test]
    fn double_ended_iterator_demo() {
        println!("Forward iteration:");
        let mut iter = MyDoubleEndedIterator::new(vec![1, 2, 3, 4, 5]);
        while let Some(item) = iter.next() {
            println!("{}", item);
        }
    
        println!("Backward iteration:");
        let mut iter_back = MyDoubleEndedIterator::new(vec![1, 2, 3, 4, 5]);
        while let Some(item) = iter_back.next_back() {
            println!("{}", item);
        }
    }    
}
