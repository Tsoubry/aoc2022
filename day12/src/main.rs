pub mod data;

use crate::data::*;
use pathfinding::prelude::bfs;

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
        let (x, y) = *current_point;

        let current_point_value = self.find_value(*current_point);

        let mut successors: Vec<Coordinate> = vec![];

        for y_coordinate in 0..self.size_y {
            for x_coordinate in 0..self.size_x {
                let value = self.find_value((x_coordinate, y_coordinate));

                let new_point = (x_coordinate, y_coordinate);

                // no idea how to do it in another way
                let options = vec![
                    (Some(x + 1), Some(y)),
                    (x.checked_sub(1), Some(y)),
                    (Some(x), Some(y + 1)),
                    (Some(x), y.checked_sub(1)),
                ]
                .iter()
                .filter(|x| x.0.is_some() && x.1.is_some())
                .map(|x| (x.0.unwrap(), x.1.unwrap()) == new_point)
                .any(|x| x == true);

                if options {
                    // force very high value
                    if value.checked_sub(1).unwrap_or(100) == current_point_value
                        || value == current_point_value
                        || value + 1 == current_point_value
                    {
                        successors.push((x_coordinate, y_coordinate))
                    }
                }
            }
        }

        successors
    }
}

fn answer_part1(data: &str) -> usize {
    let map = Map::from_data(data);

    let result = bfs(
        &map.start, 
        |p| map.find_successors(p), 
        |p| *p == map.goal);

    println!("{:?}", &result);

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
