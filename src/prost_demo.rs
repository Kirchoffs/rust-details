#[cfg(test)]
mod test {
    use bytes::{BufMut, BytesMut};
    use prost::{encode_length_delimiter, decode_length_delimiter};

    #[test]
    fn length_delimiter_demo() {
        let secret_number = 42;
        let value = "Hello, Rust!".as_bytes();

        let mut encoded_value = BytesMut::new();
        encode_length_delimiter(secret_number, &mut encoded_value).unwrap();
        encoded_value.extend_from_slice(value);
        let encoded_value = encoded_value.to_vec();

        let encoded_value_copy = encoded_value.clone();
        println!("Encoded value: {:?}", String::from_utf8_lossy(&encoded_value_copy));

        let mut decoded_value = BytesMut::new();
        decoded_value.put_slice(&encoded_value);
        let secret_number = decode_length_delimiter(&mut decoded_value).unwrap();
        println!("Secret number: {}", secret_number);
        println!("Decoded value: {}", String::from_utf8_lossy(&decoded_value));
    }
}
