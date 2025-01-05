#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    #[test]
    fn btree_map_demo() {
        let mut map = BTreeMap::<Vec<u8>, i32>::new();
        map.insert(vec![1, 255], 1);
        map.insert(vec![1, 255, 0], 2);
        map.insert(vec![2], 3);

        for (k, v) in map.iter() {
            println!("{:?} -> {}", k, v);
        }
    }

    #[test]
    fn btree_map_range_demo() {
        let mut map = BTreeMap::<Vec<u8>, i32>::new();
        map.insert(vec![1, 255], 1);
        map.insert(vec![1, 255, 1], 2);
        map.insert(vec![1, 255, 2], 3);
        map.insert(vec![1, 255, 3], 4);
        map.insert(vec![2], 5);

        let range = map.range(vec![1, 255, 1]..vec![2]);
        for (k, v) in range {
            println!("{:?} -> {}", k, v);
        }
    }
}
