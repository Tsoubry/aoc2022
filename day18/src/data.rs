pub type Coordinate = (i8, i8, i8);

pub fn import_data(data: &str) -> Vec<Coordinate> {
    data.lines().map(|line| parse(line)).collect()
}

pub fn parse(line: &str) -> Coordinate {
    let mut points = line
        .split_terminator(",")
        .map(|point| point.parse::<i8>().unwrap());

    (
        points.next().unwrap(),
        points.next().unwrap(),
        points.next().unwrap(),
    )
}

pub const TEST_DATA: &str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"#;

mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA);
        println!("{:?}", input_data);
    }
}
