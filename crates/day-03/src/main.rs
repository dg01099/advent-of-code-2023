use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_input(file_path: String) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    Ok(Box::new(BufReader::new(File::open(file_path)?)))
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PartTypes {
    Number,
    Part,
}

#[derive(Clone)]
pub struct Part {
    pub sign: String,
    pub row: usize,
    pub col_start: Option<usize>,
    pub col_end: Option<usize>,
    pub part_type: PartTypes,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: {} in row: {} from {} to {}",
            self.part_type, self.sign, self.row, self.col_start.unwrap(), self.col_end.unwrap()
        )
    }
}

impl Part {
    pub fn is_adjacent(&self, other: &Part) -> bool {
        if self.row + 1 < other.row || other.row < self.row - 1 {
            return false
        }
        if self.col_end.unwrap() + 1 < other.col_start.unwrap() || other.col_end.unwrap() < self.col_start.unwrap() - 1 {
            return false
        }
        return true
    }

    pub fn empty(row_count: usize, part_types: PartTypes) -> Result<Self, Error> {
        Ok(Part{
            sign: String::new(),
            row: row_count,
            col_start: None,
            col_end: None,
            part_type: part_types
        })
    }
}

pub fn parse_part_list(reader: Box<dyn BufRead>) -> Result<(Vec<Part>, Vec<Part>), Error> {
    let mut parts: Vec<Part> = Vec::new();
    let mut numbers: Vec<Part> = Vec::new();

    for (row_count, lines) in reader.lines().enumerate() {

        let mut number = Part::empty(row_count, PartTypes::Number)?;
        let mut part = Part::empty(row_count, PartTypes::Part)?;

        for (col_count, c) in lines.unwrap().chars().enumerate() {
            if c.is_digit(10) {
                if number.col_start.is_none() {
                    number.col_start = Some(col_count);
                }
                number.sign.push(c);
                number.col_end = Some(col_count);

            } else if number.col_start.is_some() {
                numbers.push(number.clone());
                number = Part::empty(row_count, PartTypes::Number)?
            }

            if !c.is_digit(10) && c != '.' {
                if part.col_start.is_none() {
                    part.col_start = Some(col_count);
                }
                part.sign.push(c);
                part.col_end = Some(col_count);
            } else if part.col_start.is_some() {
                parts.push(part.clone());
                part = Part::empty(row_count, PartTypes::Part)?
            }
        }

        if number.col_start.is_some() {
            numbers.push(number.clone());
            number = Part::empty(row_count, PartTypes::Number)?
        }
        if part.col_start.is_some() {
            parts.push(part.clone());
            part = Part::empty(row_count, PartTypes::Part)?
        }
    }
    Ok((numbers,parts))
}


pub fn part_one(reader: Box<dyn BufRead>) -> Result<i32, Error> {
    let mut result: i32 = 0;
    let (numbers,parts) = parse_part_list(reader)?;

    for number in numbers.iter() {
        println!("{number}");
        if parts.iter().any(|part| part.is_adjacent(number)) {
            let number_value: i32 = number.sign.parse().unwrap();
            result += number_value;
        }
    }

    println!("Sum is {result}");
    Ok(result)
}

pub fn part_two(reader: Box<dyn BufRead>) -> Result<i32, Error> {
    let mut result: i32 = 0;
    Ok(result)
}


fn main() {
    if let Ok( reader) = read_input("crates/day-03/test_input.txt".to_string()) {
        let _= part_one(reader);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let reader = read_input("crates/day-03/test_input.txt".to_string()).unwrap();
        let result = part_one(reader);
        assert_eq!(result.unwrap(), 4361);
    }

    #[test]
    fn test_part_two() {
        let reader = read_input("crates/day-03/test_input.txt".to_string()).unwrap();
        let result = part_two(reader);
        assert_eq!(result.unwrap(), 4361);
    }
}
