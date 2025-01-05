#[cfg(test)]
mod tests {
    #[test]
    fn struct_reference_lifetime() {
        struct Wrapper<'a> {
            data: &'a mut i32
        }

        let mut x = 1;
        
        {
            let mut wrapper = Wrapper { data: &mut x };
            wrapper.data = &mut 42;
            
            let y = &mut x;
            *y = 89;

            let another_wrapper = Wrapper { data: &mut x };
            *another_wrapper.data = 6174;
        }
        
        println!("{}", x);
    }

    #[test]
    fn primitive_reference_lifetime() {
        let mut x = 1;
        let y = &mut x;
        *y = 42;

        let z = &mut x;
        *z = 89;
        println!("{}", x);
    }
}
