use std::fmt::Error;

pub fn get_winning_possibilities(time: &u64, distance: &u64) -> Vec<u64> {
    let mut acceleration_time:Vec<u64> = Vec::new();
    for id in 0..*time {
        let raced_distance = id * (time - id);
        if raced_distance > *distance {
            acceleration_time.push(id);
        }

    }
    println!("Race {time} / {distance} -> You can win {}x", acceleration_time.len());
    return acceleration_time;
}


pub fn part_one(times: &Vec<u64>, distances: &Vec<u64>) -> Result<u64, Error> {
    let mut result: u64 = 1;
    for (count,time) in times.iter().enumerate() {
        result *= get_winning_possibilities(time, &distances[count]).len() as u64
    }
    if result == 1 {
        Err(Error)
    } else {
        println!("Result -> {result}");
        Ok(result)
    }

}

pub fn part_two(times: &Vec<u64>, distances: &Vec<u64>) -> Result<u64, Error> {
    let mut result: u64 = 0;
    Ok(result)
}


fn main() {
    let times = vec![44, 70, 70, 80];
    let distances = vec![283, 1134, 1134, 1491];
    let _ = part_one(&times, &distances);

    let times = vec![44707080];
    let distances = vec![283113411341491];
    let _ = part_one(&times, &distances);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let times = vec![7, 15, 30];
        let distances = vec![9, 40, 200];
        let result = part_one(&times, &distances);
        assert_eq!(result.unwrap(), 288);
    }

    #[test]
    fn test_part_two() {
        let times = vec![71530];
        let distances = vec![940200];
        let result = part_one(&times, &distances);
        assert_eq!(result.unwrap(), 71503);
    }
}