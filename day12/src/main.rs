pub mod data;

use crate::data::*;
use pathfinding::prelude::bfs;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Map {
    fn find_value(&self, coordinate: Coordinate) -> usize {
        *self
            .grid
            .get(coordinate.1)
            .expect("out of bounds y")
            .get(coordinate.0)
            .expect("out of bounds x")
    }

    pub fn find_successors(&self, current_point: &Coordinate) -> Vec<Coordinate> {
        let Coordinate(x, y) = *current_point;

        let current_point_value = self.find_value(current_point.clone());

        let mut successors: Vec<Coordinate> = vec![];

        for direction in DIRECTIONS {
            // checked_add_signed is in nightly only
            let new_x = usize::try_from((x as isize) + direction.1).ok();
            let new_y = usize::try_from((y as isize) + direction.0).ok();

            if let (Some(x_coordinate), Some(y_coordinate)) = (new_x, new_y) {
                if x_coordinate < self.size_x && y_coordinate < self.size_y {
                    let value = self.find_value(Coordinate(x_coordinate, y_coordinate));

                    if value.checked_sub(1).unwrap_or(0) <= current_point_value {
                        successors.push(Coordinate(x_coordinate, y_coordinate));
                    }
                }
            }
        }
        successors
    }
}

fn answer_part1(data: &str) -> usize {
    let map = Map::from_data(data);

    let result = bfs(&map.start, |p| map.find_successors(p), |p| *p == map.goal);

    result.expect("no path found").len() - 1
}

// fn answer_part2(data: Vec<Parsed>) -> i64 {

// }

fn main() {
    let input_data = include_str!("../input.txt");

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    // println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::data::TEST_DATA;

    #[test]
    fn test_answer1() {
        assert_eq!(31, answer_part1(TEST_DATA));
    }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(, answer_part2(input_data));
    // }

    #[test]
    fn playground() {}
}
