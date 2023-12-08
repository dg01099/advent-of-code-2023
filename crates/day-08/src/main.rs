use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use num::integer::lcm;

#[derive(Debug)]
pub struct Instruction {
    left: String,
    right: String
}

impl Instruction {
    pub fn get(&self, direction: &char) -> String {
        match direction {
            'L' => return (&self.left).to_string(),
            'R' => return (&self.right).to_string(),
            _ => panic!("Why!?")
        }
    }
}

pub fn read_input(file_path: String) -> Result<(String, HashMap<String, Instruction>), Error> {
    let reader = Box::new(BufReader::new(File::open(file_path)?));

    let mut instructions = String::new();
    let mut map: HashMap<String, Instruction> = HashMap::new();

    for (c, lines) in reader.lines().enumerate() {
        let line = lines.unwrap();
        if c == 0 {
            instructions = line;
        } else if !line.is_empty() {
            let (start, directions)  = line.split_once("=").unwrap();
            let start = start.trim().to_string();

            let directions = directions.replace(['(', ')', ' '], "");
            let (left, right) = directions
                .split_once(',')
                .unwrap();

            map.insert(start, Instruction {
                left: left.to_string(),
                right: right.to_string()
            });

        }
    }
    Ok((instructions,map))
}


pub fn part_one(instructions: &String, map: &HashMap<String, Instruction>) -> Result<u64, Error> {

    let mut result: u64 = 0;
    let mut position: String = "AAA".to_string();
    let instruction_len = instructions.len();
    let mut instruction_count: usize = 0;

    println!("Start positions: {:?}", position);

    while position != "ZZZ".to_string() {

        let direction = instructions.chars().nth(instruction_count).unwrap();
        position = map.get(&position).unwrap().get(&direction);

        // increase counter
        instruction_count += 1;
        result += 1;

        if instruction_count >= instruction_len {
            println!("Turn {result} -> Position {position:?}");
            instruction_count = 0;
        }
    }
    println!("Turn {result} -> Position {position:?}");
    println!("Result: {result}");
    Ok(result)
}

pub fn part_two(instructions: &String, map: &HashMap<String, Instruction>) -> Result<u64, Error> {

    let mut result:u64 = 1;
    let mut loop_count: u64 = 0;

    let instruction_len = instructions.len();
    let mut instruction_count: usize = 0;

    let mut positions: Vec<String> = map.keys().map(|k|k.to_string()).filter(|k| k.ends_with('A')).collect();
    let mut loop_counts: Vec<u64> = positions.iter().map(|_|0u64).collect();

    println!("Start positions: {:?}", positions);

    while loop_counts.iter().any(|f|*f == 0) {

        let start_positions = positions.clone();
        let direction = instructions.chars().nth(instruction_count).unwrap();
        positions.clear();

        // increase counter
        instruction_count += 1;
        loop_count += 1;

        for (i,pos) in start_positions.iter().enumerate() {
            let instruction = map.get(pos).unwrap();
            let next_pos = instruction.get(&direction);

            positions.push(next_pos.clone());

            if next_pos.ends_with('Z') {
                loop_counts[i] = loop_count.clone();
            }
        }

        if instruction_count >= instruction_len {
            instruction_count = 0;
        }
    }
    println!("Turn {loop_count} -> Position {positions:?} -> loop_counts {loop_counts:?}");

    for count in loop_counts {
        result = lcm(result, count as u64)
    }
    println!("Result: {result}");
    Ok(result)
}


fn main() {

    let (instructions, map) = read_input("crates/day-08/input.txt".to_string()).unwrap();

    let _= part_one(&instructions, &map);
    let _= part_two(&instructions, &map);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let (instructions, map) = read_input("test_input.txt".to_string()).unwrap();
        let result = part_one(&instructions, &map);
        assert_eq!(result.unwrap(), 6);
    }

    #[test]
    fn test_part_two() {
        let (instructions, map) = read_input("test_input_part2.txt".to_string()).unwrap();
        let result = part_two(&instructions, &map);
        assert_eq!(result.unwrap(), 6);
    }
}
