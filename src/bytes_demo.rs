#[cfg(test)]
mod test {
    use bytes::{BytesMut, BufMut};

    #[test]
    fn vec_u8_demo() {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(b"Hello, ");
        buffer.extend_from_slice(b"World!");

        let s = String::from_utf8(buffer).expect("Found invalid UTF-8");

        println!("{}", s);
    }

    #[test]
    fn bytes_mut_demo() {
        let mut buffer = BytesMut::with_capacity(64);

        buffer.put_slice(b"Hello, ");
        buffer.put_slice(b"World!");
    
        let s = String::from_utf8(buffer.to_vec()).expect("Found invalid UTF-8");
        println!("{}", s);
    }

    #[test]
    fn next_vec_of_byte_without_common_prefix_demo() {
        // let mut v = "Hello".to_string().into_bytes();
        let mut v = "Hello".as_bytes().to_vec();

        while let Some(last) = v.iter_mut().last() {
            if *last == u8::MAX {
                v.pop();
            } else {
                *last += 1;
                break;
            }
        }
        println!("{:?}", String::from_utf8(v));
    }
}
