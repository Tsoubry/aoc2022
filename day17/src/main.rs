pub mod data;

use itertools::Itertools;

use crate::data::*;

const DASH_PARTS: [(usize, usize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const PLUS_PARTS: [(usize, usize); 5] = [(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
const CORNER_PARTS: [(usize, usize); 5] = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
const PIPE_PARTS: [(usize, usize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const CUBE_PARTS: [(usize, usize); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

const EMPTY_ROW: [u8; 7] = [0; 7];

const GRID_HEIGHT: usize = 100_000;

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

    #[inline]
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
    pub grid: [[u8; 7]; GRID_HEIGHT],
    pub highest_rock: usize,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            grid: [[0; 7]; GRID_HEIGHT],
            highest_rock: 0,
        }
    }
}

impl Grid {
    #[inline(always)]
    fn move_direction(&self, rock: &mut Vec<(usize, usize)>, direction: &Direction) {
        let x_modifier: isize = match direction {
            Direction::Left => {
                let group = rock.iter().group_by(|(_, y)| y);

                let mut leftmost_parts = group.into_iter().map(|group_item| {
                    (
                        group_item
                            .1
                            .map(|group| group.0)
                            .min()
                            .expect("min on x should always work"),
                        *group_item.0,
                    )
                });

                let is_leftmost = leftmost_parts.any(|part| {
                    part.0 == 0 || self.grid[part.1][part.0.checked_sub(1).unwrap_or(0)] == 1u8
                });

                if is_leftmost {
                    0
                } else {
                    -1
                }
            }
            Direction::Right => {
                let group = rock.iter().group_by(|(_, y)| y);

                let mut rightmost_parts = group.into_iter().map(|group_item| {
                    (
                        group_item
                            .1
                            .map(|group| group.0)
                            .max()
                            .expect("max on x should always work"),
                        *group_item.0,
                    )
                });

                let is_rightmost = rightmost_parts
                    .any(|part| part.0 == 6 || self.grid[part.1][(part.0 + 1).min(6)] == 1u8);

                if is_rightmost {
                    0
                } else {
                    1
                }
            }
        };

        rock.iter_mut()
            .for_each(|(x, _)| *x = (*x as isize + x_modifier) as usize);
    }

    #[inline]
    fn move_down(&self, rock: &mut Vec<(usize, usize)>) {
        rock.iter_mut().for_each(|(_, y)| *y -= 1);
    }

    #[inline]
    fn calculate_highest_rock(&self) -> usize {
        let start_looking = self.highest_rock.max(1);
        for height in start_looking..=self.grid.len() {
            if self.grid[height] == EMPTY_ROW {
                return height - 1;
            }
        }

        0
    }

    #[inline(always)]
    fn sense_bottom_and_keep(&mut self, rock: &Vec<(usize, usize)>) -> bool {
        let group = rock.iter().group_by(|(x, _)| x);

        let mut bottom_parts = group.into_iter().map(|group_item| {
            (
                *group_item.0,
                group_item
                    .1
                    .map(|group| group.1)
                    .min()
                    .expect("min should always work"),
            )
        });

        let is_at_bottom =
            bottom_parts.any(|part| part.1 - 1 == 0 || self.grid[part.1 - 1][part.0] == 1u8);
        if is_at_bottom {
            for (x, y) in rock {
                self.grid[*y][*x] = 1;
            }

            self.highest_rock = self.calculate_highest_rock();
        }

        is_at_bottom
    }

    #[inline(never)]
    fn print(&self) {
        let start = self.highest_rock.checked_sub(5).unwrap_or(0);
        let end = self.highest_rock + 7;

        for row in (start..=end).rev() {
            println!("{:?}", self.grid[row]);
        }

        println!();
    }

    #[inline]
    fn reset_grid(&mut self) -> usize {
        let keep_end = self.highest_rock;
        let keep_start = keep_end - 500;

        let slice_to_keep = self.grid[keep_start..=keep_end].iter().enumerate();

        let mut new_grid = [[0; 7]; GRID_HEIGHT];

        for (pos, row) in slice_to_keep {
            new_grid[pos] = *row;
        }

        self.grid = new_grid;

        self.highest_rock = 0;
        self.highest_rock = self.calculate_highest_rock();

        keep_start
    }
}

fn simulate(data: Vec<Direction>, total_rocks: usize, careful: bool) -> usize {
    let mut total_height: usize = 0;

    let rocks: [Rock; 5] = [Rock::Dash, Rock::Plus, Rock::Corner, Rock::Pipe, Rock::Cube];
    let mut current_rock_pos: usize = 0;

    let pattern = data;
    let pattern_size = pattern.len();
    let mut current_pattern_pos: usize = 0;

    let mut grid = Grid::default();

    let mut rocks_fallen: usize = 0;

    while rocks_fallen < total_rocks {
        let current_rock = rocks[current_rock_pos];
        let mut rock_parts = current_rock.parts(grid.highest_rock);

        loop {
            // 1. move direction
            // 2. check if down, => keep in grid
            // 3. move down

            let current_direction = pattern
                .get(current_pattern_pos)
                .expect("pattern position out of bounds");

            grid.move_direction(&mut rock_parts, current_direction);

            if current_pattern_pos == pattern_size - 1 {
                current_pattern_pos = 0
            } else {
                current_pattern_pos += 1
            };

            if grid.sense_bottom_and_keep(&rock_parts) {
                rocks_fallen += 1;
                break;
            }

            grid.move_down(&mut rock_parts);
        }

        if current_rock_pos == 4 {
            current_rock_pos = 0
        } else {
            current_rock_pos += 1
        };

        if careful {
            if grid.highest_rock >= (GRID_HEIGHT - 500) {
                let height_to_add = grid.reset_grid();

                total_height += height_to_add;

                println!("Total height: {}", &total_height);
            }
        }
    }

    total_height += grid.highest_rock;

    if careful {
        return total_height;
    }

    grid.highest_rock
}

fn answer_part1(data: Vec<Direction>) -> usize {
    let total_rocks = 2022;

    simulate(data, total_rocks, false)
}

fn answer_part2(data: Vec<Direction>) -> usize {
    let total_rocks: usize = 1_000_000_000_000;

    simulate(data, total_rocks, true)
}

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    println!("Answer of part 2 is: {}", answer_part2(input_data));
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

    #[test]
    fn test_answer2() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(1_514_285_714_288, answer_part2(input_data));
    }
}
