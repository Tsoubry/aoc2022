pub type Path = Vec<Coordinate>;

pub type Coordinate = (usize, usize);

pub fn import_data(data: &str) -> Vec<Path> {
    data.lines().map(|line| parse(line)).collect()
}

fn parse_coordinate_point(point: Option<&str>) -> usize {
    point
        .expect("no next item in raw coordinate")
        .parse()
        .expect("unable to parse to usize for raw coordinate")
}

pub fn parse(line: &str) -> Path {
    line.split_terminator(" -> ")
        .into_iter()
        .map(|raw_coordinate| {
            let mut iter = raw_coordinate.split_terminator(",").into_iter();

            (
                parse_coordinate_point(iter.next()),
                parse_coordinate_point(iter.next()),
            )
        })
        .collect()
}

pub const TEST_DATA: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA);
        // println!("{:?}", &input_data);

        let x = input_data.iter().flatten().map(|x| x.0);
        println!(
            "Min x: {:?}, Max x: {:?}",
            x.clone().min().unwrap(),
            x.max().unwrap()
        );

        let y = input_data.iter().flatten().map(|x| x.1);
        println!(
            "Min y: {:?}, Max y: {:?}",
            y.clone().min().unwrap(),
            y.max().unwrap()
        );
    }

    #[test]
    fn test_parse_full_file() {
        let input_data = import_data(include_str!("../input.txt"));
        // println!("{:?}", &input_data);

        let x = input_data.iter().flatten().map(|x| x.0);
        println!(
            "Min x: {:?}, Max x: {:?}",
            x.clone().min().unwrap(),
            x.max().unwrap()
        );

        let y = input_data.iter().flatten().map(|x| x.1);
        println!(
            "Min y: {:?}, Max y: {:?}",
            y.clone().min().unwrap(),
            y.max().unwrap()
        );
    }
}
