use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Bytes, Error};
use std::str::FromStr;



pub fn read_input(file_path: String) -> Result<(Vec<String>, Vec<u64>), Error> {
    let reader = Box::new(BufReader::new(File::open(file_path)?));
    let mut hands: Vec<String> = Vec::new();
    let mut bids: Vec<u64> = Vec::new();

    for lines in reader.lines() {
        if let Ok(line) = lines {
            let (hand,bid) = line.split_once(" ").unwrap();
            hands.push(hand.clone().to_string());
            bids.push(bid.trim().parse().unwrap());
        }
    }
    Ok((hands,bids))
}

#[derive(Debug, Clone, Copy)]
enum Games {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Debug, Clone)]
pub struct Hand {
    cards: String,
    game: Games,
    value: u64,
    bid: u64
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.trim().to_string();
        let game = Hand::get_game(&cards);

        Ok(Hand {
            cards: cards.clone(),
            game: game.clone(),
            value: Hand::calculate_value(&game, &cards),
            bid: 0
        })
    }
}

impl Hand {

    fn get_game(cards: &String) -> Games{
        let mut card_count: HashMap<char,u8> = HashMap::new();
        for card in cards.chars() {
           *card_count.entry(card).or_default() += 1;
        }
        let mut pairs:Vec<u8> = vec![0;5];
        for count in card_count.values() {
            pairs[(*count - 1) as usize] += 1;
        }

        if pairs[4] == 1 {
            return Games::FiveOfKind
        } else if pairs[3] == 1 {
            return Games::FourOfKind
        } else if pairs[2] == 1 && pairs[1] == 1 {
            return Games::FullHouse
        } else if pairs[2] == 1 {
            return Games::ThreeOfKind
        } else if pairs[1] == 2 {
            return Games::TwoPair
        } else if pairs[1] == 1 {
            return Games::OnePair
        }
        return Games::HighCard
    }

    fn calculate_value(game: &Games, cards: &String) -> u64 {
        let mut value:u64 = 0;

        match game {
            Games::FiveOfKind => value = 32 * 10_u64.pow(12),
            Games::FourOfKind => value = 16 * 10_u64.pow(12),
            Games::FullHouse => value = 8 * 10_u64.pow(12),
            Games::ThreeOfKind => value = 4 * 10_u64.pow(12),
            Games::TwoPair => value = 2 * 10_u64.pow(12),
            Games::OnePair => value = 1 * 10_u64.pow(12),
            Games::HighCard => value = 0 * 10_u64.pow(12),
            _ => panic!("Unknown Game!")
        }

        for (i,card) in cards.chars().enumerate() {
            let index = i as u32;
            match card {
                'A' => value += 12 * 10_u64.pow( 9 - (2*index)),
                'K' => value += 11 * 10_u64.pow( 9 - (2*index)),
                'Q' => value += 10 * 10_u64.pow( 9 - (2*index)),
                'J' => value += 9 * 10_u64.pow( 9 - (2*index)),
                'T' => value += 8 * 10_u64.pow( 9 - (2*index)),
                '9' => value += 7 * 10_u64.pow( 9 - (2*index)),
                '8' => value += 6 * 10_u64.pow( 9 - (2*index)),
                '7' => value += 5 * 10_u64.pow( 9 - (2*index)),
                '6' => value += 4 * 10_u64.pow( 9 - (2*index)),
                '5' => value += 3 * 10_u64.pow( 9 - (2*index)),
                '4' => value += 2 * 10_u64.pow( 9 - (2*index)),
                '3' => value += 1 * 10_u64.pow( 9 - (2*index)),
                '2' => value += 0 * 10_u64.pow( 9 - (2*index)),
                _ => panic!("Unknown Card!")

            }
        }
        return value
    }
}

pub fn part_one(cards:&Vec<String>, bids:&Vec<u64>) -> Result<u64, Error> {
    let mut result: u64 = 0;
    let mut hands: Vec<Hand> = Vec::new();

    for (id, card) in cards.iter().enumerate() {
        let mut hand = Hand::from_str(card).unwrap();
        hand.bid = bids[id];
        hands.push(hand.clone());
        // println!("{:?}", hand);
    }
    hands.sort_by_key(|f|f.value);
    for (rank,hand) in hands.iter().enumerate(){
        result += hand.bid * (rank as u64 + 1);
        println!("{:?} {:?}",rank as u64 + 1, hand);
    }
    // let mut ranks = arg_sort(&hand_values);
    // println!("{ranks:?}");
    // let result: u64  = bids.iter().zip(ranks.iter())
    //     .map(|(bid, rank)| bid * (*rank as u64 + 1))
    //     .sum();

    println!("Result: {result}");
    Ok(result)
}

pub fn part_two(cards:&Vec<String>, bids:&Vec<u64>) -> Result<u64, Error> {
    let mut result: u64 = 0;
    Ok(result)
}


fn main() {

    let (cards, bids) = read_input("crates/day-07/input.txt".to_string()).unwrap();

    let _= part_one(&cards, &bids);
    let _= part_two(&cards, &bids);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let (cards, bids) = read_input("test_input.txt".to_string()).unwrap();
        let result = part_one(&cards, &bids);
        assert_eq!(result.unwrap(), 6440);
    }

    #[test]
    fn test_part_two() {
        let (cards, bids) = read_input("test_input.txt".to_string()).unwrap();
        let result = part_two(&cards, &bids);
        assert_eq!(result.unwrap(), 5905);
    }
}
