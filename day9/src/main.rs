pub mod coordinate;

use strum_macros::EnumString;
#[macro_use]
extern crate derive_new;

use crate::coordinate::Coordinate;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, EnumString, PartialEq)]
pub enum Direction {
    D,
    U,
    L,
    R,
}

pub type Instruction = (Direction, u8);

fn import_data(data: &str) -> Vec<Instruction> {
    data.lines().map(|line| parse(line)).collect()
}

fn parse(line: &str) -> Instruction {
    let mut splitted = line.split_whitespace();
    let dir = Direction::from_str(splitted.next().unwrap()).unwrap();
    let amount: u8 = splitted.next().unwrap().parse().unwrap();

    (dir, amount)
}

fn answer_part1(data: Vec<Instruction>) -> usize {
    let mut positions_visited: HashMap<Coordinate, u32> = HashMap::new();

    let mut head_position = Coordinate::new(0, 0);
    let mut tail_position = Coordinate::new(0, 0);
    positions_visited.insert(tail_position, 1);

    for (direction, distance) in data {
        for _ in 0..distance {
            head_position = head_position.walk(&direction);
            if head_position.check_coordinate_around(&tail_position) {
                // don't need to do anything
            } else {
                if head_position.get_x() != tail_position.get_x()
                    && head_position.get_y() != tail_position.get_y()
                {
                    tail_position = tail_position.walk_diagonally(&direction, &head_position);
                } else {
                    tail_position = tail_position.walk(&direction);
                }

                positions_visited
                    .entry(tail_position)
                    .and_modify(|amount| *amount += 1)
                    .or_insert(1);
            }
        }
    }

    positions_visited
        .into_iter()
        .filter(|(_, value)| value >= &1)
        .count()
}

fn answer_part2(data: Vec<Instruction>) -> usize {
    let mut positions_visited: HashMap<Coordinate, u32> = HashMap::new();

    let empty_coordinate = Coordinate::new(0, 0);

    let mut knots = vec![empty_coordinate.clone()]
        .into_iter()
        .cycle()
        .take(10)
        .collect::<Vec<_>>();

    let last_knot = knots.get(9).unwrap();
    positions_visited.insert(*last_knot, 1);

    // just initialization
    let mut head_position = empty_coordinate.clone();
    let mut next_knot = empty_coordinate.clone();

    for (direction, distance) in data {
        for _ in 0..distance {
            head_position = head_position.walk(&direction);
            knots[0] = head_position;

            for index in 1..10usize {
                next_knot = *knots.get(index).unwrap();

                if head_position.check_coordinate_around(&next_knot) {
                    // don't need to do anything
                } else {
                    next_knot = next_knot.walk_towards(&head_position);

                    if index == 9 {
                        positions_visited
                            .entry(next_knot)
                            .and_modify(|amount| *amount += 1)
                            .or_insert(1);
                    }
                }

                // end: update positions in vec and variables

                knots[index] = next_knot;
                head_position = next_knot;
            }

            // loop finishes, reset head_position
            head_position = *knots.get(0).unwrap();
        }

        // println!("state after direction: {:?} {}, {:?}", &direction, &distance, &knots.iter().map(|x| x.to_string()).collect::<Vec<_>>());
    }

    // println!("{:?}", &positions_visited);

    positions_visited
        .into_iter()
        .filter(|(_, value)| value >= &1)
        .count()
}

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    const TEST_DATA2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    #[test]
    fn test_parsing() {
        let _input_data = import_data(TEST_DATA);
        // println!("{:?}", _input_data);
    }

    #[test]
    fn test_answer1() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(13, answer_part1(input_data));
    }

    #[test]
    fn test_answer2() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(1, answer_part2(input_data));

        let input_data2 = import_data(TEST_DATA2);
        assert_eq!(36, answer_part2(input_data2));
    }

    #[test]
    fn playground() {}
}
