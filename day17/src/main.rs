pub mod data;

#[macro_use]
extern crate derive_new;

use crate::data::*;


const TOTAL_ROCKS: usize = 2022;

const DASH_PARTS: [(usize, usize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const PLUS_PARTS: [(usize, usize); 5] = [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
const CORNER_PARTS: [(usize, usize); 5] = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
const PIPE_PARTS: [(usize, usize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const CUBE_PARTS: [(usize, usize); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

trait Rock {

    #[inline(always)]
    fn modify_rock_position(&self, parts: &[(usize, usize)], highest_rock: usize) -> Vec<(usize, usize)> {
        parts.iter().map(|pos| (pos.0 + 2, pos.1 + 4 + highest_rock)).collect()
    }

    #[inline(always)]
    fn parts(&self, highest_rock: usize)-> Vec<(usize, usize)>;
}

#[derive(new)]
struct Dash {}
#[derive(new)]
struct Plus {}
#[derive(new)]
struct Corner {}
#[derive(new)]
struct Pipe {}
#[derive(new)]
struct Cube {}


impl Rock for Dash {
    fn parts(&self, highest_rock: usize)-> Vec<(usize, usize)> {
        self.modify_rock_position(&DASH_PARTS, highest_rock)
    }
}

impl Rock for Plus {
    fn parts(&self, highest_rock: usize)-> Vec<(usize, usize)> {
        self.modify_rock_position(&PLUS_PARTS, highest_rock)
    }
}

impl Rock for Corner {
    fn parts(&self, highest_rock: usize)-> Vec<(usize, usize)> {
        self.modify_rock_position(&CORNER_PARTS, highest_rock)
    }
}

impl Rock for Pipe {
    fn parts(&self, highest_rock: usize)-> Vec<(usize, usize)> {
        self.modify_rock_position(&PIPE_PARTS, highest_rock)
    }
}

impl Rock for Cube {
    fn parts(&self, highest_rock: usize)-> Vec<(usize, usize)> {
        self.modify_rock_position(&CUBE_PARTS, highest_rock)
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
            highest_rock: 0
        }
    }

}

impl Grid {

    fn sense_colision(&self, rock: Vec<(usize, usize)>) -> bool {
        todo!()
    }


}




fn answer_part1(data: Vec<Direction>) -> usize {

    let rocks: Vec<Box<dyn Rock>> = vec![
        Box::new(Dash::new()),
        Box::new(Plus::new()),
        Box::new(Corner::new()),
        Box::new(Pipe::new()),
        Box::new(Cube::new()),
        ];





    0
}

// fn answer_part2(data: Vec<Parsed>) -> i64 {

// }

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    // println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    // println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::data::TEST_DATA;

    // #[test]
    // fn test_answer1() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(3068, answer_part1(input_data));
    // }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(, answer_part2(input_data));
    // }

    #[test]
    fn playground() {
        
    }
}
