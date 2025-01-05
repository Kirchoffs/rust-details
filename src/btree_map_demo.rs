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
}
