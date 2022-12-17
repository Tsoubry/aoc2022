#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

pub fn import_data(data: &str) -> Vec<Direction> {
    let first_line = data.lines().next().unwrap();

    parse(first_line)
}

pub fn parse(line: &str) -> Vec<Direction> {
    line.chars()
        .map(|character| match character {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("found character that was not expected."),
        })
        .collect()
}

pub const TEST_DATA: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
"#;

mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let pattern = import_data(TEST_DATA);
        // println!("{:?}", pattern);
    }
}
