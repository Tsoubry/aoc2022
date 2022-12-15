use regex::Regex;

#[derive(new, Debug, Clone)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

pub fn import_data(data: &str) -> Vec<(Coordinate, Coordinate)> {
    data.lines().map(|line| parse(line)).collect()
}

fn text_to_coordinate(text: &str) -> Coordinate {
    let re = Regex::new(r".+at x=(\-?\d+), y=(-?\d+)").unwrap();
    let captures = re.captures(text).expect("no captures found");

    let x = captures
        .get(1)
        .expect("first capture group")
        .as_str()
        .parse::<isize>()
        .expect("parse to isize");
    let y = captures
        .get(2)
        .expect("second capture group")
        .as_str()
        .parse::<isize>()
        .expect("parse to isize");

    Coordinate::new(x, y)
}

fn parse(line: &str) -> (Coordinate, Coordinate) {
    let mut iter = line
        .split_terminator(": ")
        .map(|text| text_to_coordinate(text));

    (iter.next().unwrap(), iter.next().unwrap())
}

pub const TEST_DATA: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA);
        // println!("{:?}", input_data);
    }
}
