pub mod data;

use crate::data::*;

const TOTAL_ROCKS: usize = 2022;

const DASH_PARTS: [(usize, usize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const PLUS_PARTS: [(usize, usize); 5] = [(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
const CORNER_PARTS: [(usize, usize); 5] = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
const PIPE_PARTS: [(usize, usize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const CUBE_PARTS: [(usize, usize); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

#[derive(Clone, Copy)]
enum Rock {
    Dash,
    Plus,
    Corner,
    Pipe,
    Cube,
}

impl Rock {
    #[inline(always)]
    fn modify_rock_position(
        &self,
        parts: &[(usize, usize)],
        highest_rock: usize,
    ) -> Vec<(usize, usize)> {
        parts
            .iter()
            .map(|pos| (pos.0 + 2, pos.1 + 4 + highest_rock))
            .collect()
    }

    fn parts(&self, highest_rock: usize) -> Vec<(usize, usize)> {
        match self {
            Rock::Dash => self.modify_rock_position(&DASH_PARTS, highest_rock),
            Rock::Plus => self.modify_rock_position(&PLUS_PARTS, highest_rock),
            Rock::Corner => self.modify_rock_position(&CORNER_PARTS, highest_rock),
            Rock::Pipe => self.modify_rock_position(&PIPE_PARTS, highest_rock),
            Rock::Cube => self.modify_rock_position(&CUBE_PARTS, highest_rock),
        }
    }
}

struct Grid {
    pub grid: [[u8; 7]; 10000],
    pub highest_rock: usize,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            grid: [[0; 7]; 10000],
            highest_rock: 0,
        }
    }
}

impl Grid {
    #[inline(always)]
    fn move_direction(&self, rock: Vec<(usize, usize)>, direction: &Direction) {
        // todo: needs to keep outside boundaries of fallen rocks as well
        let x_modifier: isize = match direction {
            Direction::Left => {
                if rock.first().unwrap().0 == 0 {
                    0
                } else {
                    -1
                }
            }
            Direction::Right => {
                if rock.last().unwrap().1 == 6 {
                    0
                } else {
                    1
                }
            }
        };

        // todo: mut
    }

    fn move_down(&self, rock: &mut Vec<(usize, usize)>) {

        rock
        .iter_mut()
        .for_each(|(_, y)| { *y -= 1 });

    }

    #[inline(always)]
    fn sense_bottom_and_keep(&mut self, rock: Vec<(usize, usize)>) -> bool {

        // min max calc

        // also check for floor!

        // also update highest rock

        true
    }

}

fn answer_part1(data: Vec<Direction>) -> usize {
    let rocks: [Rock; 5] = [Rock::Dash, Rock::Plus, Rock::Corner, Rock::Pipe, Rock::Cube];
    let mut current_rock_pos: usize = 0;

    let pattern = data;
    let pattern_size = pattern.len();
    let mut current_pattern_pos: usize = 0;

    let mut grid = Grid::default();

    let mut rocks_fallen: usize = 0;

    while rocks_fallen < TOTAL_ROCKS {
        let current_rock = rocks[current_rock_pos];
        let mut rock_parts = current_rock.parts(grid.highest_rock);

        loop {

            let current_direction = pattern.get(current_pattern_pos).expect("pattern position out of bounds");

            // 1. move direction
            // 2. check if down, => keep in grid
            // 3. move down

            // if stuck: rocks fallen += 1, break
            if grid.sense_bottom_and_keep(rock_parts) {
                rocks_fallen += 1;
                break;
            }

            if current_pattern_pos == pattern_size {
                current_pattern_pos = 0
            } else {
                current_pattern_pos += 1
            };


        }

        if current_rock_pos == 4 {
            current_rock_pos = 0
        } else {
            current_rock_pos += 1
        };
    }

    grid.highest_rock
}

// fn answer_part2(data: Vec<Parsed>) -> i64 {

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
    fn test_answer1() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(3068, answer_part1(input_data));
    }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(, answer_part2(input_data));
    // }

    #[test]
    fn playground() {}
}
