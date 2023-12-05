use std::fs::File;
use std::io::{BufRead, BufReader, Bytes, Error};


pub fn read_input(file_path: String) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    Ok(Box::new(BufReader::new(File::open(file_path)?)))
}

pub fn part_one(reader: Box<dyn BufRead>) -> Result<i32, Error> {
    let mut result: i32 = 0;
    Ok(result)
}

pub fn part_two(reader: Box<dyn BufRead>) -> Result<i32, Error> {
    let mut result: i32 = 0;
    Ok(result)
}


fn main() {
    // if let Ok( reader) = read_input("crates/day-0/input.txt".to_string()) {
    //     let _= part_one(reader);
    // }
    if let Ok( reader) = read_input("crates/day-0/input.txt".to_string()) {
        let _= part_two(reader);
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_part_one() {
            let reader = read_input("test_input.txt".to_string()).unwrap();
            let result = part_one(reader);
            assert_eq!(result.unwrap(), 35);
        }

        #[test]
        fn test_part_two() {
            let reader = read_input("test_input.txt".to_string()).unwrap();
            let result = part_two(reader);
            assert_eq!(result.unwrap(), 467835);
        }
    }
