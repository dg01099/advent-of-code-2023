use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn read_input() -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    Ok(Box::new(BufReader::new(File::open("crates/day-01/input.txt")?)))
}

fn part_one() -> Result<(), Box<dyn std::error::Error>> {

    let reader = read_input()?;
    let mut total_sum = 0;

    for lines in reader.lines() {
        let line: String = lines.unwrap();
        let mut digits_per_line: Vec<char> = Vec::new();
        for char in line.chars() {
            if char.is_digit(10) {
                digits_per_line.push(char)
            }
        }

        let mut number_as_string = String::new();

        if let Some(first) = digits_per_line.first() {
            number_as_string.push(*first);
        }
        if let Some(last) = digits_per_line.last() {
            number_as_string.push(*last);
        }

        let number: i32 = number_as_string.parse().unwrap();
        total_sum += number;

        print!("Line: {line} -> {digits_per_line:?} -> {number} -> {total_sum} \n");
    }

    Ok(())
}
fn check_is_number(input: String) -> Result<char, String> {

    if input.contains("one") {
        return Ok('1')
    } else if input.contains("two") {
        return Ok('2')
    }else if input.contains("three") {
        return Ok('3')
    }else if input.contains("four") {
        return Ok('4')
    }else if input.contains("five") {
        return Ok('5')
    }else if input.contains("six") {
        return Ok('6')
    }else if input.contains("seven") {
        return Ok('7')
    }else if input.contains("eight") {
        return Ok('8')
    }else if input.contains("nine") {
        return Ok('9')
    }
    return Err("Is no number!".to_string())
}

fn part_two() -> Result<(), Box<dyn std::error::Error>> {

    let reader = read_input()?;
    let mut total_sum = 0;

    for (n, lines) in reader.lines().enumerate() {
        let line: String = lines.unwrap();

        let mut number_as_string = String::new();
        let mut string_first = String::new();
        let mut string_last = String::new();

        for char in line.chars() {
            if char.is_digit(10) {
                number_as_string.push(char);
                break
            } else {
                string_first.push(char);
                if let Ok(res) = check_is_number(string_first.clone()) {
                    number_as_string.push(res);
                    break
                }
            }
        }

        for char in line.chars().rev() {
            if char.is_digit(10) {
                number_as_string.push(char);
                break
            } else {
                string_last.insert(0, char);
                if let Ok(res) = check_is_number(string_last.clone()) {
                    number_as_string.push(res);
                    break
                }
            }
        }

        let number: i32 = number_as_string.parse().unwrap();
        total_sum += number;

        print!("Line #{n}: {line} -> {number} -> {total_sum} \n");
    }

    Ok(())

}

fn main() {
    // part_one();
    part_two();
}