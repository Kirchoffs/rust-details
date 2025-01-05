#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Result;

    #[test]
    fn buf_read_test() -> Result<()> {
        let file_path = "resources/input.txt";
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            println!("{}", line);
        }

        Ok(())
    }
}
