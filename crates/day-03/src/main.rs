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
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(read_input("crates/day-03/test_input.txt".to_string())?);
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(read_input("crates/day-03/test_input.txt".to_string())?);
        assert_eq!(result, Some(281));
    }
}