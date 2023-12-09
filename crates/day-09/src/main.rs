use std::fs::File;
use std::io::{BufRead, BufReader, Bytes, Error};

pub fn calc_diff(values: &Vec<i32>) -> Vec<i32>{
    let mut diffs: Vec<i32> = Vec::new();
    for (i,value) in values.iter().enumerate() {
        if i+1 < values.len() {
            diffs.push(values[i+1] - value)
        }
    }
    return diffs
}

pub fn calc_successor_value(values: Vec<i32>) -> i32 {
    let mut value_diff_map: Vec<Vec<i32>> = Vec::new();
    value_diff_map.push(values.clone());

    let mut diff = calc_diff(&values);
    loop {
        if diff.iter().all(|v| *v == 0) {
            break;
        }
        println!("{:?}", diff);
        value_diff_map.push(diff.clone());
        diff = calc_diff(&diff);
    }

    let mut next_value: i32 = 0;
    value_diff_map.reverse();
    for diff_map in value_diff_map  {
        next_value += diff_map.last().unwrap();
    }
    println!("Next value is {next_value}");
    return next_value
}


pub fn calc_previous_value(values: Vec<i32>) -> i32 {
    let mut value_diff_map: Vec<Vec<i32>> = Vec::new();
    value_diff_map.push(values.clone());

    let mut diff = calc_diff(&values);
    loop {
        if diff.iter().all(|v| *v == 0) {
            break;
        }
        println!("{:?}", diff);
        value_diff_map.push(diff.clone());
        diff = calc_diff(&diff);
    }

    let mut prev_value: i32 = 0;
    value_diff_map.reverse();
    for diff_map in value_diff_map  {
        prev_value = diff_map.first().unwrap() - prev_value;
    }
    println!("Previous value is {prev_value}");
    return prev_value
}


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
    for row in values {
        result += calc_successor_value(row);
    }
    println!("Result: {result}");
    Ok(result)
}

pub fn part_two(values: Vec<Vec<i32>>) -> Result<i32, Error> {
    let mut result: i32 = 0;
    for row in values {
        result += calc_previous_value(row);
    }
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
        assert_eq!(result.unwrap(), 114);
    }

    #[test]
    fn test_part_two() {
        let values = read_input("test_input.txt".to_string()).unwrap();
        let result = part_two(values);
        assert_eq!(result.unwrap(), 2);
    }
}
