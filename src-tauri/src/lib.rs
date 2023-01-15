pub mod file_utils {

    use std::{path::Path, io::{self, BufRead}, fs::File};

    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

#[macro_export]
macro_rules! unwrap_or_return {
    ( $e:expr, $msg:expr ) => {
        match $e {
            Some(x) => x,
            None => return Err(io::Error::new(Other, $msg)),
        }
    }
}

#[macro_export]
macro_rules! result_unwrap_or_return {
    ( $e:expr, $msg:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return Err(io::Error::new(Other, $msg)),
        }
    }
}