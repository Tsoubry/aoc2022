pub mod data;

use crate::data::*;

const STARTING_POINT: (usize, usize) = (500, 0);

struct Grid<const N: usize> {
    pub grid: [[u8; N]; N],
    pub max_depth: usize,
}

impl<const N: usize> Grid<N> {

    fn from_paths(paths: Vec<Path>) -> Self {
        let mut grid = [[0; N]; N];
        let mut max_depth: usize = 0;

        for path in paths {
            let mut path_iter = path.into_iter();

            let mut start = path_iter.next().expect("First path is empty");

            for next in path_iter {
                if next.0 == start.0 {
                    if next.1 > max_depth {
                        max_depth = next.1;
                    }

                    if next.1 > start.1 {
                        for pos in start.1..(next.1 + 1) {
                            grid[pos][next.0] = 1;
                        }
                    } else {
                        for pos in next.1..(start.1 + 1) {
                            grid[pos][next.0] = 1;
                        }
                    }
                } else {
                    if next.0 > start.0 {
                        for pos in start.0..(next.0 + 1) {
                            grid[next.1][pos] = 1;
                        }
                    } else {
                        for pos in next.0..(start.0 + 1) {
                            grid[next.1][pos] = 1;
                        }
                    }
                }

                start = next;
            }
        }

        Grid { grid, max_depth }
    }
}

fn answer_part1(data: Vec<Path>) -> usize {
    let mut grid = Grid::<1000>::from_paths(data);

    println!("max depth: {}", grid.max_depth);
    
    let mut current_x = STARTING_POINT.0;
    let mut current_y = STARTING_POINT.1;

    let mut total_units: usize = 0;

    loop {

        if current_y > grid.max_depth {
            break
        }
        
        if grid.grid
        .get(current_y + 1).unwrap()
        .get(current_x).expect("Out of bounds x!")
        != &1 {
            current_y += 1;
        } else if grid.grid
        .get(current_y + 1).unwrap()
        .get(current_x.checked_sub(1).expect("subtract from usize")).expect("Out of bounds x!") != &1 {
            current_y += 1;
            current_x -= 1;
        } else if grid.grid
        .get(current_y + 1).unwrap()
        .get(current_x + 1).expect("Out of bounds x!") != &1 {
            current_y += 1;
            current_x += 1;
        } else {
            let update_point = grid.grid.get_mut(current_y).unwrap().get_mut(current_x).expect("Out of bounds x!");
            *update_point = 1;

            total_units += 1;

            current_x = STARTING_POINT.0;
            current_y = STARTING_POINT.1;
        }

    }

    total_units
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
    fn test_grid_conversion() {

        let input_data = import_data(TEST_DATA);

        let grid = Grid::<1000>::from_paths(input_data);

        assert_eq!(9, grid.max_depth);
        assert_eq!(1, grid.grid[7][502]);
        assert_eq!(1, grid.grid[9][500]);
        assert_eq!(1, grid.grid[4][498]);
        assert_eq!(20, grid.grid.iter().flatten().map(|x| *x as u32).sum::<u32>());

    // println!("Grid:");
    // for row in &grid.grid[0..10] {
    //     println!("{:?}", row.get(494..504));
    // };

    }

    #[test]
    fn test_answer1() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(24, answer_part1(input_data));
    }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(, answer_part2(input_data));
    // }

}
