const RADIX: u32 = 10;
const MATRIX_SIZE: usize = 99;

#[derive(Debug)]
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

    fn find_max_x_1(&self, x: usize, y: usize) -> Result<u8, CoordinateError> {

        self.grid[x][0..y].iter().max().ok_or(CoordinateError).copied()
    }

    fn find_max_x_2(&self, x: usize, y: usize) -> u8 {

        *self.grid[x][y..].iter().max().unwrap()
    }

    fn find_max_y_1(&self, x: usize, y: usize) -> u8 {
        *self.grid[0..x][y].iter().max().unwrap()
    }

    fn find_max_y_2(&self, x: usize, y: usize) -> u8 {
        *self.grid[x..][y].iter().max().unwrap()
    }

    fn decide_visible(&self, x: Option<usize>, y: Option<usize>, center_value: u8) -> bool {
        if x.is_none() || y.is_none() {
            return true;
        };

        let xu = x.unwrap();
        let yu = y.unwrap();

        match self.get_coordinate_value(xu, yu) {
            Ok(_) => {
                if self.find_max_x_1(xu, yu) >= center_value
                    && self.find_max_x_2(xu, yu) >= center_value
                    && self.find_max_y_1(xu, yu) >= center_value
                    && self.find_max_y_2(xu, yu) >= center_value
                {
                    false
                } else {
                    true
                }
            }
            Err(_) => true,
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
    let mut visible: Vec<u8> = vec![];

    for col_number in 0..N {
        for row_number in 0..N {
            let current_value = grid.get_coordinate_value(row_number, col_number).unwrap();

            grid.decide_visible(row_number.checked_sub(1), Some(col_number), current_value);

            let up =
                grid.decide_visible(row_number.checked_sub(1), Some(col_number), current_value);
            let down = grid.decide_visible(Some(row_number + 1), Some(col_number), current_value);
            let left =
                grid.decide_visible(Some(row_number), col_number.checked_sub(1), current_value);
            let right = grid.decide_visible(Some(row_number), Some(col_number + 1), current_value);

            if up || down || left || right {
                total_visible += 1;
                visible.push(current_value);
            }
        }
    }

    println!("{:?}", visible);

    total_visible
}

// fn answer_part2<const N: usize>(grid: Grid<N>) -> u32 {

// }

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    let mut grid = Grid::<MATRIX_SIZE>::default();
    grid.from_data(input_data);

    println!(
        "Answer of part 1 is: {}",
        answer_part1::<MATRIX_SIZE>(grid.clone())
    );
    // println!("Answer of part 2 is: {}", answer_part2(input_data));
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
    }

    #[test]
    fn test_answer1() {
        let input_data = import_data(TEST_DATA);

        let mut grid = Grid::<5>::default();
        grid.from_data(input_data);

        assert_eq!(21, answer_part1(grid));
    }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(, answer_part2(input_data));
    // }

    #[test]
    fn playground() {}
}
