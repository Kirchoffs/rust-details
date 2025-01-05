#[cfg(test)]
mod test {
    use std::{marker::PhantomPinned, pin};

    use futures::executor::block_on;

    #[test]
    fn basic() {
        struct U<'a> {
            x: String,
            y: i32,
            z: &'a i32
        }

        let kaprekar_num = 6174;
        let mut u = U {
            x: String::from("abc"),
            y: 42,
            z: &kaprekar_num
        };
        println!("{:p}", &u);
        println!("{:p}", &u.x);
        println!("{:p}", &u.y);
        println!("{:p}", &u.z);
        
        println!("------");
        
        let taxicab_num = 1729;
        u = U {
            x: String::from("def"),
            y: 89,
            z: &taxicab_num
        };
        println!("{:p}", &u);
        println!("{:p}", &u.x);
        println!("{:p}", &u.y);
        println!("{:p}", &u.z);
    }

    // Same result for C++ for above case
    //
    // #include <iostream>
    // #include <string>

    // struct U {
    //     std::string x;
    //     int y;
    //     const int* z;
    // };

    // int main() {
    //     int kaprekar_num = 6174;
    //     U u = {"abc", 42, &kaprekar_num};

    //     std::cout << &u << std::endl;
    //     std::cout << &u.x << std::endl;
    //     std::cout << &u.y << std::endl;
    //     std::cout << &u.z << std::endl;

    //     std::cout << "------" << std::endl;

    //     int taxicab_num = 1729;
    //     u = {"def", 89, &taxicab_num};

    //     std::cout << &u << std::endl;
    //     std::cout << &u.x << std::endl;
    //     std::cout << &u.y << std::endl;
    //     std::cout << &u.z << std::endl;
    // }

    #[test]
    fn self_referential_structure_issue() {
        #[derive(Debug)]
        struct TestStructure {
            a: String,
            b: *const String,
        }

        impl TestStructure {
            fn new(txt: &str) -> Self {
                TestStructure {
                    a: String::from(txt),
                    b: std::ptr::null(),
                }
            }

            fn init(&mut self) {
                let self_ref: *const String = &self.a;
                self.b = self_ref;
            }

            fn a(&self) -> &str {
                &self.a
            }

            fn b(&self) -> &String {
                unsafe { &*(self.b) }
            }
        }

        let mut test1 = TestStructure::new("test1");
        test1.init();
        let mut test2 = TestStructure::new("test2");
        test2.init();
    
        println!("test1 (before swap): a: {}, b: {}", test1.a(), test1.b());
        println!("test2 (before swap): a: {}, b: {}", test2.a(), test2.b());
        std::mem::swap(&mut test1, &mut test2);
        println!("test1 (after swap): a: {}, b: {}", test1.a(), test1.b());
        println!("test2 (after swap): a: {}, b: {}", test2.a(), test2.b());
    }

    #[test]
    fn self_referential_structure_issue_solution_1() {
        use std::ptr::NonNull;

        #[derive(Debug)]
        struct InlineBuf {
            array: [u8; 64],
            slice: NonNull<[u8]>,
        }

        impl InlineBuf {
            pub fn new() -> Self {
                Self {
                    array: [0; 64],
                    slice: NonNull::from(&[])
                }
            }

            pub fn set_contents(&mut self, contents: &[u8]) -> bool {
                let contents_len = contents.len();
                if contents_len > self.array.len() {
                    return false;
                }

                self.array[..contents_len].copy_from_slice(contents);
                self.slice = NonNull::from(&self.array[..contents_len]);
                true
            }

            pub fn as_bytes(&self) -> &[u8] {
                unsafe {
                    &*self.slice.as_ptr()
                }
            }
        }

        let mut buf = InlineBuf::new();
        {
            let mut tmp_buf = InlineBuf::new();
            tmp_buf.set_contents(b"Hello, world!!!!!!");

            buf = tmp_buf;

            tmp_buf = InlineBuf::new();
            tmp_buf.set_contents(b"Hello, rust!");
        }

        let len = b"Hello, rust!".len();
        assert_eq!(buf.as_bytes()[..len], b"Hello, rust!"[..len]);
    }

    #[test]
    fn self_referential_structure_issue_solution_2() {
        use std::ptr::NonNull;
        use std::pin::Pin;

        #[derive(Debug)]
        struct InlineBuf {
            array: [u8; 64],
            slice: NonNull<[u8]>,
            _pinned: PhantomPinned,
        }

        impl InlineBuf {
            pub fn new() -> Self {
                Self {
                    array: [0; 64],
                    slice: NonNull::from(&[]),
                    _pinned: PhantomPinned,
                }
            }

            pub fn set_contents(self: Pin<&mut Self>, contents: &[u8]) -> bool {
                let contents_len = contents.len();
                if contents_len > self.array.len() {
                    return false;
                }

                unsafe {
                    let this = self.get_unchecked_mut();
                    this.array[..contents_len].copy_from_slice(contents);
                    this.slice = NonNull::from(&this.array[..contents_len]);
                }

                true
            }

            pub fn as_bytes(&self) -> &[u8] {
                unsafe {
                    &*self.slice.as_ptr()
                }
            }
        }

        {
            let mut buf = Box::pin(InlineBuf::new());
            let mut_buf = buf.as_mut();
            mut_buf.set_contents(b"Hello, world!");

            assert_eq!(buf.as_bytes(), b"Hello, world!");
        }

        {
            use std::pin::pin;
            let buf = InlineBuf::new();
            let mut pinned_buf = pin!(buf);
            pinned_buf.as_mut().set_contents(b"Hello, rust!");

            assert_eq!(pinned_buf.as_bytes(), b"Hello, rust!");
        }
    }
}
