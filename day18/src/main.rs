pub mod data;

use crate::data::*;

#[derive(Clone, Copy)]
pub struct Cube {
    coordinate: Coordinate,
    sides_open: usize,
}

impl Cube {
    fn new(coordinate: Coordinate) -> Self {
        Self {
            coordinate,
            sides_open: 6,
        }
    }
}

fn answer_part1(data: Vec<Coordinate>) -> usize {
    let mut cubes: Vec<_> = data.clone().into_iter().map(|pos| Cube::new(pos)).collect();

    for coordinate in data {
        for cube in cubes.iter_mut() {
            if coordinate != cube.coordinate {
                if ((coordinate.0, coordinate.1) == (cube.coordinate.0, cube.coordinate.1)
                    && ((cube.coordinate.2 - 1)..=(cube.coordinate.2 + 1)).contains(&coordinate.2))
                    || ((coordinate.0, coordinate.2) == (cube.coordinate.0, cube.coordinate.2)
                        && ((cube.coordinate.1 - 1)..=(cube.coordinate.1 + 1))
                            .contains(&coordinate.1))
                    || ((coordinate.1, coordinate.2) == (cube.coordinate.1, cube.coordinate.2)
                        && ((cube.coordinate.0 - 1)..=(cube.coordinate.0 + 1))
                            .contains(&coordinate.0))
                {
                    cube.sides_open -= 1;
                }
            }
        }
    }

    cubes.iter().map(|cube| cube.sides_open).sum()
}

// fn answer_part2(data: Vec<Coordinate>) -> usize {

// }

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    // println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::data::TEST_DATA;

    #[test]
    fn test_simple() {
        let input_data = vec![(1, 1, 1), (2, 1, 1)];
        assert_eq!(10, answer_part1(input_data));
    }

    #[test]
    fn test_answer1() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(64, answer_part1(input_data));
    }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(, answer_part2(input_data));
    // }

    #[test]
    fn playground() {}
}
