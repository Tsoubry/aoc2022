use std::collections::HashMap;

pub type Parsed = Vec<usize>;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coordinate(pub usize, pub usize);

#[derive(Debug, Clone)]
pub struct Map {
    pub grid: Vec<Vec<usize>>,
    pub start: Coordinate,
    pub goal: Coordinate,
    pub size_y: usize,
    pub size_x: usize,
}

impl Map {
    pub fn from_data(data: &str) -> Self {
        let alphabet = ('a'..='z')
            .into_iter()
            .enumerate()
            .map(|(number, letter)| (letter, number))
            .collect::<HashMap<char, usize>>();

        let mut grid: Vec<Vec<usize>> = vec![];
        let mut start: Coordinate = Coordinate(0, 0);
        let mut goal: Coordinate = Coordinate(0, 0);

        for (y_coordinate, line) in data.lines().enumerate() {
            let mut letters: Vec<usize> = vec![];
            for (x_coordinate, character) in line.chars().enumerate() {
                match character {
                    'S' => {
                        start = Coordinate(x_coordinate, y_coordinate);
                        letters.push(*alphabet.get(&'a').expect("character not found in alphabet"));
                    }
                    'E' => {
                        goal = Coordinate(x_coordinate, y_coordinate);
                        letters.push(*alphabet.get(&'z').expect("character not found in alphabet"));
                    }
                    any => {
                        letters.push(*alphabet.get(&any).expect("character not found in alphabet"));
                    }
                }
            }

            grid.push(letters);
        }

        let size_y = grid.len();
        let size_x = grid.first().unwrap().len();

        Self {
            grid,
            start,
            goal,
            size_y,
            size_x,
        }
    }
}

pub const TEST_DATA: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let _map = Map::from_data(TEST_DATA);
        // println!("{:?}", _map);
    }

    #[test]
    fn test_parse_all_data() {
        let _map = Map::from_data(include_str!("../input.txt"));
        // println!("{:?}", _map);
    }
}
