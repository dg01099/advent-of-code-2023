use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use regex::Regex;

pub enum ParseGameError {
    Comment,
    InvalidFormat
}
#[derive(Debug)]
pub struct Game {
    pub count: i32,
    pub blue: Vec<i32>,
    pub red: Vec<i32>,
    pub green: Vec<i32>,
}


pub fn read_input(file_path: String) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    Ok(Box::new(BufReader::new(File::open(file_path)?)))
}

pub fn extract_game_infos_from_line(line: &str) -> Result<Game, Error> {
    let line_parts = line.split_once(':').unwrap();
    let game_count: i32 = line_parts.0.to_string()
        .replace("Game ", "")
        .parse().unwrap();

    let games_str = line_parts.1.to_string();
    let color = Regex::new(r"(?P<number>\d+) (?P<color>(blue|green|red))").unwrap();

    let mut new_game = Game{
        count: game_count,
        green: Vec::new(),
        blue: Vec::new(),
        red: Vec::new()
    };

    for game in games_str.split(';').map(str::trim).collect::<Vec<_>>() {

        let mut g: i32 = 0;
        let mut b: i32 = 0;
        let mut r: i32 = 0;

        for colors in color.captures_iter(game) {

            let color = colors.name("color").unwrap().as_str();
            let count:i32 = colors.name("number").unwrap().as_str().parse().unwrap();

            match color{
                "green" => g+= count,
                "blue" => b+= count,
                "red" => r+= count,
                _ => ()
            }
        }
        new_game.green.push(g);
        new_game.blue.push(b);
        new_game.red.push(r);
    }
    return Ok(new_game)
}

pub fn part_one(reader: Box<dyn BufRead>, max_red: i32, max_green: i32, max_blue: i32) -> Result<i32, Error> {
    let mut result: i32 = 0;

    for (count, lines) in reader.lines().enumerate() {
        let mut line = lines.unwrap();

        if let Ok(game) = extract_game_infos_from_line(line.as_str()) {

            if game.red.iter().any(|value| value > &max_red) {
                continue
            }
            if game.green.iter().any(|value| value > &max_green) {
                continue
            }
            if game.blue.iter().any(|value| value > &max_blue) {
                continue
            }

            result += game.count;
            print!("Line #{count}: {line} -> {game:?} -> {result}\n");
        }
    }
    Ok(result)
}

pub fn part_two(reader: Box<dyn BufRead>) -> Result<i32, Error> {
    let mut result: i32 = 0;

    for (count, lines) in reader.lines().enumerate() {
        let mut line = lines.unwrap();
        if let Ok(game) = extract_game_infos_from_line(line.as_str()) {
            let power_green = game.green.iter().max().unwrap();
            let power_blue = game.blue.iter().max().unwrap();
            let power_red = game.red.iter().max().unwrap();

            result += (power_green*power_blue*power_red);
            print!("Line #{count}: {line} -> {game:?} -> {result}\n");
        }


    }

    Ok(result)
}


fn main() {
    // if let Ok( reader) = read_input("crates/day-02/input.txt".to_string()) {
    //     let _= part_one(reader, 12, 13, 14);
    // }
    if let Ok( reader) = read_input("crates/day-02/input.txt".to_string()) {
        let _= part_two(reader);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(read_input("crates/day-02/test_input.txt".to_string())?, 12, 13, 14);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(read_input("crates/day-02/test_input.txt".to_string())?);
        assert_eq!(result, Some(281));
    }
}