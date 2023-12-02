pub fn import_data(data: &str) -> Vec<isize> {
    data.lines()
        .filter_map(|line| line.parse::<isize>().ok())
        .collect()
}

pub const TEST_DATA: &str = r#"1
2
-3
3
-2
0
4
"#;

mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA);
        // println!("{:?}", input_data);
    }
}
