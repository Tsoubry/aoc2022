pub mod coordinate;

use strum_macros::EnumString;
#[macro_use]
extern crate derive_new;


use std::str::FromStr;
use std::collections::HashMap;
use crate::coordinate::Coordinate;

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
                if head_position.get_x() != tail_position.get_x() && head_position.get_y() != tail_position.get_y() {
                    tail_position = tail_position.walk_diagonally(&direction, &head_position);
                } else {
                    tail_position = tail_position.walk(&direction); 
                }

                positions_visited.entry(tail_position).and_modify(|amount| *amount += 1).or_insert(1);
            }

        }

    }

    positions_visited
    .into_iter()
    .filter(|(_, value)| value >= &1)
    .count()

}

// fn answer_part2(data: Vec<Instruction>) -> i64 {

// }

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    // println!("Answer of part 2 is: {}", answer_part2(input_data));
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
    

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA);
        println!("{:?}", input_data);
    }

    #[test]
    fn test_answer1() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(13, answer_part1(input_data));
    }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(, answer_part2(input_data));
    // }

    #[test]
    fn playground() {
        
    }
}
