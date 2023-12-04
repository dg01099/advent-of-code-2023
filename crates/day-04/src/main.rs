use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::str::FromStr;

pub fn read_input(file_path: String) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    Ok(Box::new(BufReader::new(File::open(file_path)?)))
}

pub enum ParseCardError {
    Comment,
    InvalidFormat,
}

pub struct Card {
    id: i32,
    winning_numbers: HashSet<i8>,
    my_numbers: HashSet<i8>
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card,numbers) = s.split_once(':').ok_or(ParseCardError::Comment)?;
        let card_id:i32 = card.replace("Card", "").trim().parse().unwrap();

        let (win_str, num_str) = numbers.split_once('|').unwrap();

        let win_numbers = win_str.split(" ").filter(|&f| !f.is_empty()).map(|s| s.parse().unwrap()).collect();
        let my_numbers = num_str.split(" ").filter(|&f| !f.is_empty()).map(|s| s.parse().unwrap()).collect();

        return Ok(Card{
            id: card_id,
            winning_numbers: win_numbers,
            my_numbers
        })
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Card {}: winning numbers: {:?} | my numbers {:?}",
            self.id, self.winning_numbers, self.my_numbers
        )
    }
}

pub fn part_one(reader: Box<dyn BufRead>) -> Result<i32, Error> {
    let mut result: i32 = 0;
    let mut card_stack: Vec<Card> = Vec::new();

    for lines in reader.lines() {
        if let Ok(line) = lines {
            if let Ok(card) = Card::from_str(&line) {
                card_stack.push(card);
            }
        }
    }
    for card in card_stack.iter() {
        let number_count = card.my_numbers.intersection(&card.winning_numbers).count();
        let multiplier: i32 = 2;
        let mut value = 0;
        if number_count > 0 {
            value = multiplier.pow(number_count as u32 - 1);
        }
        result += value;
        println!("{card} -> Value: {value} -> Result {result}");
    }
    Ok(result)
}

pub fn part_two(reader: Box<dyn BufRead>) -> Result<i32, Error> {
    let mut result: i32 = 0;
    let mut card_stack: Vec<Card> = Vec::new();

    for lines in reader.lines() {
        if let Ok(line) = lines {
            if let Ok(card) = Card::from_str(&line) {
                card_stack.push(card);
            }
        }
    }

    for card in card_stack.iter() {
        let number_count = card.my_numbers.intersection(&card.winning_numbers).count();
        let multiplier: i32 = 2;
        let mut value = 0;
        if number_count > 0 {
            value = multiplier.pow(number_count as u32 - 1);
        }
        result += value;
        println!("{card} -> Value: {value} -> Result {result}");
    }
    Ok(result)
}



fn main() {
    if let Ok(reader) = read_input("crates/day-04/input.txt".to_string()) {
        let _ = part_one(reader);
    }
    // if let Ok( reader) = read_input("crates/day-0/input.txt".to_string()) {
    //     let _= part_two(reader);
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let reader = read_input("test_input.txt".to_string()).unwrap();
        let result = part_one(reader);
        assert_eq!(result.unwrap(), 13);
    }

    #[test]
    fn test_part_two() {
        let reader = read_input("test_input.txt".to_string()).unwrap();
        let result = part_two(reader);
        assert_eq!(result.unwrap(), 30);
    }
}
