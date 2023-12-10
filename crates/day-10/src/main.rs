use std::fs::File;
use std::io::{BufRead, BufReader, Error};


pub fn read_input(file_path: String) -> Result<Vec<Vec<i32>>,Error> {
    let reader = Box::new(BufReader::new(File::open(file_path)?));
    let mut values: Vec<Vec<i32>> = Vec::new();

    for lines in reader.lines() {
        let line = lines.unwrap();
        let line_values: Vec<i32> = line.split(" ")
            .into_iter()
            .map(|v| v.parse().unwrap())
            .collect();
        values.push(line_values.clone());
    }
    Ok(values)
}

pub fn part_one(values: Vec<Vec<i32>>) -> Result<i32, Error> {
    let mut result: i32 = 0;
    println!("Result: {result}");
    Ok(result)
}

pub fn part_two(values: Vec<Vec<i32>>) -> Result<i32, Error> {
    let mut result: i32 = 0;
    println!("Result: {result}");
    Ok(result)
}


fn main() {
    if let Ok( values) = read_input("crates/day-09/input.txt".to_string()) {
        let _= part_two(values);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let values = read_input("test_input.txt".to_string()).unwrap();
        let result = part_one(values);
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn test_part_two() {
        let values = read_input("test_input.txt".to_string()).unwrap();
        let result = part_two(values);
        assert_eq!(result.unwrap(), 2);
    }
}
