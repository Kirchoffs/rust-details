#[cfg(test)]
mod test {
    // copied() method focus on the primitive type, actually all the types that implement the Copy trait
    // mainly for shallow copy
    #[test]
    fn option_copied_demo() {
        let val = 42;
        
        let opt: Option<&i32> = Some(&val);
        let opt_copied: Option<i32> = opt.copied();
        
        match opt_copied {
            Some(num) => println!("Copied value: {}", num),
            None => println!("No value"),
        }
    }

    // cloned() method focus on the reference type, actually all the types that implement the Clone trait
    // mainly for deep copy
    #[test]
    fn option_cloned_demo() {
        let mut map = std::collections::HashMap::new();
        map.insert(1, vec![1]);

        let one_vec_option: Option<&Vec<i32>> = map.get(&1);
        let one_vec: Option<Vec<i32>> = one_vec_option.cloned();
        println!("one_vec: {:?}", one_vec);
    }

    #[test]
    fn result_option_cloned_demo() {
        let val = 42;

        // From Result<&i32, &str> to Result<i32, &str>
        let res: Result<&i32, &str> = Ok(&val);
        let res_cloned: Result<i32, &str> = res.cloned();
        println!("res_cloned: {:?}", res_cloned);

        // From Option<&i32> to Option<i32>
        let opt: Option<&i32> = Some(&val);
        let opt_cloned: Option<i32> = opt.cloned();
        println!("opt_cloned: {:?}", opt_cloned);
    }
}