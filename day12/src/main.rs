pub mod data;

use crate::data::*;
use pathfinding::prelude::bfs;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Map {
    fn find_value(&self, coordinate: &Coordinate) -> usize {
        *self
            .grid
            .get(coordinate.1)
            .expect("out of bounds y")
            .get(coordinate.0)
            .expect("out of bounds x")
    }

    fn find_successors(&self, current_point: &Coordinate) -> Vec<Coordinate> {
        let Coordinate(x, y) = *current_point;

        let current_point_value = self.find_value(&current_point);

        let mut successors: Vec<Coordinate> = vec![];

        for direction in DIRECTIONS {
            // checked_add_signed is in nightly only, so use this instead
            let new_x = usize::try_from((x as isize) + direction.1).ok();
            let new_y = usize::try_from((y as isize) + direction.0).ok();

            if let (Some(x_coordinate), Some(y_coordinate)) = (new_x, new_y) {
                if x_coordinate < self.size_x && y_coordinate < self.size_y {
                    let point = Coordinate(x_coordinate, y_coordinate);
                    let value = self.find_value(&point);

                    if value.checked_sub(1).unwrap_or(0) <= current_point_value {
                        successors.push(point);
                    }
                }
            }
        }
        successors
    }

    fn find_lowest_points(&self) -> Vec<Coordinate> {
        let mut points: Vec<Coordinate> = vec![];

        for y_coordinate in 0..self.size_y {
            for x_coordinate in 0..self.size_x {
                let point = Coordinate(x_coordinate, y_coordinate);
                if self.find_value(&point) == 0 {
                    points.push(point);
                }
            }
        }

        points
    }
}

fn answer_part1(data: &str) -> usize {
    let map = Map::from_data(data);

    let result = bfs(&map.start, |p| map.find_successors(p), |p| *p == map.goal);

    result.expect("no path found").len() - 1
}

fn answer_part2(data: &str) -> usize {
    let map = Map::from_data(data);

    let lowest_points = map.find_lowest_points();

    let mut shortest_path: usize = 1_000_000;

    for starting_point in lowest_points {
        let result = bfs(
            &starting_point,
            |p| map.find_successors(p),
            |p| *p == map.goal,
        )
        .map(|outcome| outcome.len() - 1);

        if let Some(path_length) = result {
            if path_length < shortest_path {
                shortest_path = path_length;
            }
        }
    }

    shortest_path
}

fn main() {
    let input_data = include_str!("../input.txt");

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::data::TEST_DATA;

    #[test]
    fn test_answer1() {
        assert_eq!(31, answer_part1(TEST_DATA));
    }

    #[test]
    fn test_answer2() {
        assert_eq!(29, answer_part2(TEST_DATA));
    }
}
