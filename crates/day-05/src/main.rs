use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::str::FromStr;

pub fn read_input(file_path: String) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    Ok(Box::new(BufReader::new(File::open(file_path)?)))
}
#[derive(Debug)]
pub enum DayParserError {
    Comment,
    InvalidFormat,
}
#[derive(Debug)]
pub struct MapRange {
    destination: u64,
    source: u64,
    length: u64
}

impl FromStr for MapRange {

    type Err = DayParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").map(|s| s.trim()).collect();

        if parts.len() != 3 {
            return Err(DayParserError::Comment);
        }

        Ok(MapRange{
            destination: parts[0].parse().unwrap(),
            source: parts[1].parse().unwrap(),
            length: parts[2].parse().unwrap()
        })
    }
}

impl MapRange {

    pub fn is_in(&self, value: &u64) -> bool {
        let r = &self.source..&(&self.source+&self.destination);
        return r.contains(&value)
    }
    pub fn get_destination_value(&self, source_value: &u64) -> Option<u64> {
        if source_value >= &self.source && (source_value <= &(&self.source + &self.length)) {
            return Some(&self.destination + (source_value - &self.source));
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct XToYMap<'a> {
    ranges: Vec<MapRange>,
    name: &'a str
}

impl XToYMap<'_> {
    pub fn get_destination_value(&self, source_value: &u64) -> u64 {
        for range in &self.ranges {
            if let Some(dest_value) = range.get_destination_value(&source_value) {
                return dest_value;
            }
        }
        return *source_value
    }
}

pub fn map_reader(map_name: &str, reader:Box<dyn BufRead>) -> Option<XToYMap> {
    let mut is_started = false;
    let mut map = XToYMap {
        name: map_name,
        ranges: Vec::new()
    };

    for lines in reader.lines() {
        let line = lines.unwrap();

        if is_started {
            if line.is_empty() {
                break;
            } else {
                map.ranges.push(MapRange::from_str(&line).unwrap())
            }
        }

        if line.starts_with(map_name) {
            is_started = true;
        }
    }
    if map.ranges.len() == 0 {
        None
    } else {
        Some(map)
    }
}

pub fn seed_reader(reader:Box<dyn BufRead>) -> Option<Vec<u64>> {
    let mut seeds: Vec<u64> = Vec::new();
    for lines in reader.lines() {
        let line = lines.unwrap();
        if line.starts_with("seeds:") {

            seeds = line.replace("seeds:", "")
                .split(" ")
                .filter(|&f| !f.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            break;
        }
    }

    if seeds.len() == 0 {
        None
    } else {
        Some(seeds)
    }
}
pub fn seed_reader2(reader:Box<dyn BufRead>) -> Option<Vec<u64>> {
    let mut seed_numbers: Vec<u64> = Vec::new();
    for lines in reader.lines() {
        let line = lines.unwrap();
        if line.starts_with("seeds:") {

            seed_numbers = line.replace("seeds:", "")
                .split(" ")
                .filter(|&f| !f.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            break;
        }
    }

    if seed_numbers.len() % 2 != 0 {
        None
    } else {
        let mut seeds: Vec<u64> = Vec::new();
        for i in 0..seed_numbers.len() / 2 {
            let base = seed_numbers[i*2];
            let len = seed_numbers[(i*2)+1];
            for n in 0..len {
                seeds.push(base+n);
            }
        }
        Some(seeds)
    }
}

pub fn part_one(file_path: String) -> Result<u64, Error> {

    let mut result: u64 = 0;

    let map_names = vec![
        "seed-to-soil".to_string(),
        "soil-to-fertilizer".to_string(),
        "fertilizer-to-water".to_string(),
        "water-to-light".to_string(),
        "light-to-temperature".to_string(),
        "temperature-to-humidity".to_string(),
        "humidity-to-location".to_string(),
    ];
    let mut maps = map_names.
        iter().
        map(|name| map_reader(name, read_input(file_path.clone()).unwrap()).unwrap());

    let seeds = seed_reader(read_input(file_path.clone()).unwrap()).unwrap();
    println!("Seeds: {seeds:?}");

    let mut location_values:Vec<u64> = Vec::new();
    let mut source_values = seeds;

    for mapper in maps.into_iter() {
        location_values.clear();
        for source_value in source_values.iter() {
            let destination_value = mapper.get_destination_value(&source_value);
            println!("{:?} -> {} -> {}", mapper.name, source_value, destination_value);
            location_values.push(destination_value);
        }
        source_values.clear();
        source_values = location_values.clone();
    }

    println!("{:?}", location_values);
    result = *location_values.iter().min().unwrap();
    println!("Result: {result}");
    Ok(result)
}


pub fn part_two(file_path: String) -> Result<u64, Error> {

    let mut result: u64 = 0;

    let map_names = vec![
        "seed-to-soil".to_string(),
        "soil-to-fertilizer".to_string(),
        "fertilizer-to-water".to_string(),
        "water-to-light".to_string(),
        "light-to-temperature".to_string(),
        "temperature-to-humidity".to_string(),
        "humidity-to-location".to_string(),
    ];
    let mut maps = map_names.
        iter().
        map(|name| map_reader(name, read_input(file_path.clone()).unwrap()).unwrap());

    let seeds = seed_reader2(read_input(file_path.clone()).unwrap()).unwrap();

    let mut location_values:Vec<u64> = Vec::new();
    let mut source_values = seeds;

    for mapper in maps.into_iter() {
        location_values.clear();
        for source_value in source_values.iter() {
            let destination_value = mapper.get_destination_value(&source_value);
            location_values.push(destination_value);
        }
        source_values.clear();
        source_values = location_values.clone();
    }

    result = *location_values.iter().min().unwrap();
    println!("Result: {result}");
    Ok(result)
}


fn main() {
    // let _= part_one("crates/day-05/input.txt".to_string());
    let _= part_two("crates/day-05/input.txt".to_string());
    // if let Ok(reader) = read_input("crates/day-0/input.txt".to_string()) {
    //     let _ = part_two(reader);
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("test_input.txt".to_string());
        assert_eq!(result.unwrap(), 35);
    }

    #[test]
    fn test_part_two() {
        let result = part_two("test_input.txt".to_string());
        assert_eq!(result.unwrap(), 46);
    }
}
