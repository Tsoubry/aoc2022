const RADIX: u32 = 10;
const MATRIX_SIZE: usize = 99;

#[derive(Debug, PartialEq)]
struct CoordinateError;

fn import_data(data: &str) -> Vec<Vec<u8>> {
    data.lines().map(|line| parse(line)).collect()
}

fn parse(line: &str) -> Vec<u8> {
    line.chars()
        .map(|c| c.to_digit(RADIX).unwrap() as u8)
        .collect()
}

#[derive(Clone, Debug)]
struct Grid<const N: usize> {
    pub grid: [[u8; N]; N],
}

impl<const N: usize> Grid<N> {
    fn from_data(&mut self, data: Vec<Vec<u8>>) {
        for row in 0..N {
            for col in 0..N {
                self.grid[row][col] = data[row][col];
            }
        }
    }

    fn print(&self) {
        for row in self.grid {
            println!("{:?}", row);
        }
    }

    fn get_coordinate_value(&self, x: usize, y: usize) -> Result<u8, CoordinateError> {
        self.grid
            .get(y)
            .ok_or(CoordinateError)?
            .get(x)
            .ok_or(CoordinateError)
            .copied()
    }

    fn find_up(&self, x: usize, y: usize) -> Result<u8, CoordinateError> {
        self.grid
            .iter()
            .map(|s| s.iter().nth(x).unwrap())
            .take(y)
            .max()
            .ok_or(CoordinateError)
            .copied()
    }

    fn find_down(&self, x: usize, y: usize) -> Result<u8, CoordinateError> {
        self.grid
            .iter()
            .map(|s| s.iter().nth(x).unwrap())
            .skip(y + 1)
            .max()
            .ok_or(CoordinateError)
            .copied()
    }

    fn find_right(&self, x: usize, y: usize) -> Result<u8, CoordinateError> {
        self.grid
            .get(y)
            .ok_or(CoordinateError)?
            .get(x + 1..)
            .ok_or(CoordinateError)?
            .iter()
            .max()
            .ok_or(CoordinateError)
            .copied()
    }

    fn find_left(&self, x: usize, y: usize) -> Result<u8, CoordinateError> {
        self.grid
            .get(y)
            .ok_or(CoordinateError)?
            .get(..x)
            .ok_or(CoordinateError)?
            .iter()
            .max()
            .ok_or(CoordinateError)
            .copied()
    }

    fn find_viewing_distance_up(&self, x: usize, y: usize, tree_value: u8) -> u32 {
        let iter = self.grid.iter().map(|s| s.iter().nth(x).unwrap()).take(y);

        if iter.len() == 0 {
            return 0;
        } else {
            let mut trees_clear: u32 = 0;

            for tree in iter.rev() {
                if *tree < tree_value {
                    trees_clear += 1;
                } else {
                    trees_clear += 1;
                    break;
                }
            }

            return trees_clear;
        }
    }

    fn find_viewing_distance_down(&self, x: usize, y: usize, tree_value: u8) -> u32 {
        let iter = self
            .grid
            .iter()
            .map(|s| s.iter().nth(x).unwrap())
            .skip(y + 1);

        if iter.len() == 0 {
            return 0;
        } else {
            let mut trees_clear: u32 = 0;

            for tree in iter {
                if *tree < tree_value {
                    trees_clear += 1;
                } else {
                    trees_clear += 1;
                    break;
                }
            }

            return trees_clear;
        }
    }

    fn find_viewing_distance_left(&self, x: usize, y: usize, tree_value: u8) -> u32 {
        let iter = self.grid.get(y).unwrap().get(..x).unwrap().iter();

        if iter.len() == 0 {
            return 0;
        } else {
            let mut trees_clear: u32 = 0;

            for tree in iter.rev() {
                if *tree < tree_value {
                    trees_clear += 1;
                } else {
                    trees_clear += 1;
                    break;
                }
            }

            return trees_clear;
        }
    }

    fn find_viewing_distance_right(&self, x: usize, y: usize, tree_value: u8) -> u32 {
        let iter = self.grid.get(y).unwrap().get(x + 1..).unwrap().iter();

        if iter.len() == 0 {
            return 0;
        } else {
            let mut trees_clear: u32 = 0;

            for tree in iter {
                if *tree < tree_value {
                    trees_clear += 1;
                } else {
                    trees_clear += 1;
                    break;
                }
            }

            return trees_clear;
        }
    }
}

impl<const N: usize> Default for Grid<N> {
    fn default() -> Self {
        Self { grid: [[0; N]; N] }
    }
}

fn answer_part1<const N: usize>(grid: Grid<N>) -> u32 {
    let mut total_visible = 0;

    for col_number in 0..N {
        for row_number in 0..N {
            let current_value = grid.get_coordinate_value(row_number, col_number).unwrap();

            let up = grid.find_up(row_number, col_number).ok();
            let down = grid.find_down(row_number, col_number).ok();
            let left = grid.find_left(row_number, col_number).ok();
            let right = grid.find_right(row_number, col_number).ok();

            match (up, down, left, right) {
                (Some(u), Some(d), Some(l), Some(r)) => {
                    if u >= current_value
                        && d >= current_value
                        && l >= current_value
                        && r >= current_value
                    {
                    } else {
                        total_visible += 1
                    }
                }
                _ => total_visible += 1,
            };
        }
    }

    total_visible
}

fn answer_part2<const N: usize>(grid: Grid<N>) -> u32 {
    let mut highest_score: u32 = 0;

    for col_number in 0..N {
        for row_number in 0..N {
            let current_value = grid.get_coordinate_value(row_number, col_number).unwrap();

            let up = grid.find_viewing_distance_up(row_number, col_number, current_value);
            let down = grid.find_viewing_distance_down(row_number, col_number, current_value);
            let left = grid.find_viewing_distance_left(row_number, col_number, current_value);
            let right = grid.find_viewing_distance_right(row_number, col_number, current_value);

            let score = up * down * left * right;

            if score > highest_score {
                highest_score = score
            };
        }
    }

    highest_score
}

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    let mut grid = Grid::<MATRIX_SIZE>::default();
    grid.from_data(input_data);

    // grid.print();

    println!(
        "Answer of part 1 is: {}",
        answer_part1::<MATRIX_SIZE>(grid.clone())
    );
    println!("Answer of part 2 is: {}", answer_part2(grid));
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = r#"30373
25512
65332
33549
35390
"#;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA);

        let mut grid = Grid::<5>::default();
        grid.from_data(input_data);

        grid.print();

        assert_eq!(7, grid.get_coordinate_value(3, 0).unwrap());

        assert_eq!(7, grid.find_up(3, 3).unwrap());
        assert_eq!(9, grid.find_down(3, 3).unwrap());
        assert_eq!(5, grid.find_left(3, 3).unwrap());
        assert_eq!(9, grid.find_right(3, 3).unwrap());

        assert_eq!(1, grid.get_coordinate_value(3, 1).unwrap());
        assert_eq!(7, grid.find_up(3, 1).unwrap());
        assert_eq!(9, grid.find_down(3, 1).unwrap());
        assert_eq!(5, grid.find_left(3, 1).unwrap());
        assert_eq!(2, grid.find_right(3, 1).unwrap());

        assert_eq!(Err(CoordinateError), grid.find_down(4, 4));
    }

    #[test]
    fn test_answer1() {
        let input_data = import_data(TEST_DATA);

        let mut grid = Grid::<5>::default();
        grid.from_data(input_data);

        assert_eq!(21, answer_part1(grid));
    }

    #[test]
    fn test_functions_part2() {
        let input_data = import_data(TEST_DATA);

        let mut grid = Grid::<5>::default();
        grid.from_data(input_data);

        assert_eq!(3, grid.find_viewing_distance_up(3, 3, 4));

        assert_eq!(1, grid.find_viewing_distance_down(3, 3, 4));
        assert_eq!(2, grid.find_viewing_distance_down(0, 0, 3));

        assert_eq!(0, grid.find_viewing_distance_left(0, 0, 3));
        assert_eq!(1, grid.find_viewing_distance_left(1, 0, 0));

        assert_eq!(2, grid.find_viewing_distance_right(0, 0, 3));
        assert_eq!(0, grid.find_viewing_distance_right(4, 4, 0));
    }

    #[test]
    fn test_answer2() {
        let input_data = import_data(TEST_DATA);

        let mut grid = Grid::<5>::default();
        grid.from_data(input_data);

        assert_eq!(8, answer_part2(grid));
    }
}
