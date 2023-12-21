use std::fmt::Error;

pub enum FieldError {
    Comment,
    InvalidValue
}


struct Field {
    sign: char,
    pos_x: u64,
    pos_y: u64,
    distance_to_start: Option<u64>,
    accessible_neighbors: Vec<(u64, u64)>
}


impl Field {

    fn create(c: &char, pos_x: &i32, pos_y: &i32, max_x: &i32, max_y: &i32) -> Result<Self, Error> {
        let mut neighbors: Vec<(i32, i32)> = Vec::new();
        let mut accessible_neighbors: Vec<(u64, u64)> = Vec::new();
        let mut distance: Option<u64> = None;
        // get all neighbors based by type
        match c {
            'F' => neighbors.extend(vec![(0, 1), (1, 0)]),
            'L' => neighbors.extend(vec![(0, -1), (1, 0)]),
            '7' => neighbors.extend(vec![(0, 1), (-1, 0)]),
            'J' => neighbors.extend(vec![(0, -1), (-1, 0)]),
            '|' => neighbors.extend(vec![(0, 1), (0, -1)]),
            '-' => neighbors.extend(vec![(1, 0), (-1, 0)]),
            'S' => {
                // start all connecting neighbors
                neighbors.extend(vec![(1, 0), (-1, 0), (0, 1), (0, -1)]);
                distance = Some(0);
            },
            '.' => { }, // ground no neighbors
            &_ => {}
        }

        // calculate neighbor positions and remove all impossible positions
        for (i,(nx, ny)) in neighbors.iter().enumerate() {
            if nx + pos_x < 0 && nx + pos_x >= *max_x {
                continue
            }

            if ny + pos_y < 0 && ny + pos_y >=*max_y {
                continue
            }

            accessible_neighbors.push(((nx.clone() + pos_x) as u64, (ny.clone() + pos_y) as u64))
        }

        accessible_neighbors.sort();

        Ok(Field{
            sign: c.clone(),
            pos_x: pos_x.clone() as u64,
            pos_y: pos_y.clone() as u64,
            distance_to_start: distance,
            accessible_neighbors: accessible_neighbors.clone(),
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_create() {

        if let Ok(result) = Field::create(&'F', &5, &5, &10, &10) {
            assert_eq!(result.accessible_neighbors, vec![(5, 6), (6, 5)]);
            assert_eq!(result.pos_x, 5);
            assert_eq!(result.pos_y, 5);
            assert_eq!(result.sign, 'F');
            assert_eq!(result.distance_to_start, None);
        } else { panic!("Could not create") }

        if let Ok(result) = Field::create(&'L', &5, &5, &10, &10) {
            assert_eq!(result.accessible_neighbors, vec![(5, 4), (6, 5)]);
            assert_eq!(result.pos_x, 5);
            assert_eq!(result.pos_y, 5);
            assert_eq!(result.sign, 'L');
            assert_eq!(result.distance_to_start, None);
        } else { panic!("Could not create") }

        if let Ok(result) = Field::create(&'7', &5, &5, &10, &10) {
            assert_eq!(result.accessible_neighbors, vec![(4, 5), (5, 6)]);
            assert_eq!(result.pos_x, 5);
            assert_eq!(result.pos_y, 5);
            assert_eq!(result.sign, '7');
            assert_eq!(result.distance_to_start, None);
        } else { panic!("Could not create") }

        if let Ok(result) = Field::create(&'J', &5, &5, &10, &10) {
            assert_eq!(result.accessible_neighbors, vec![(4, 5), (5, 4)]);
            assert_eq!(result.pos_x, 5);
            assert_eq!(result.pos_y, 5);
            assert_eq!(result.sign, 'J');
            assert_eq!(result.distance_to_start, None);
        } else { panic!("Could not create") }

        if let Ok(result) = Field::create(&'|', &5, &5, &10, &10) {
            assert_eq!(result.accessible_neighbors, vec![(5, 4), (5, 6)]);
            assert_eq!(result.pos_x, 5);
            assert_eq!(result.pos_y, 5);
            assert_eq!(result.sign, '|');
            assert_eq!(result.distance_to_start, None);
        } else { panic!("Could not create") }

        if let Ok(result) = Field::create(&'-', &5, &5, &10, &10) {
            assert_eq!(result.accessible_neighbors, vec![(4, 5), (6, 5)]);
            assert_eq!(result.pos_x, 5);
            assert_eq!(result.pos_y, 5);
            assert_eq!(result.sign, '-');
            assert_eq!(result.distance_to_start, None);
        } else { panic!("Could not create") }

        if let Ok(result) = Field::create(&'S', &5, &5, &10, &10) {
            assert_eq!(result.accessible_neighbors, vec![(4, 5), (5, 4), (5, 6), (6, 5)]);
            assert_eq!(result.pos_x, 5);
            assert_eq!(result.pos_y, 5);
            assert_eq!(result.sign, 'S');
            assert_eq!(result.distance_to_start.unwrap(), 0);
        } else { panic!("Could not create") }


    }
}