pub type AnswerDtype = i64;

pub type Parsed = ();

pub fn import_data(data: &str) -> Vec<Parsed> {
    data.lines().map(|line| parse(line)).collect()
}

pub fn parse(line: &str) -> Parsed {
    todo!()
}

pub const TEST_DATA_1: &str = r#"
"#;
pub const TEST_ANSWER_1: AnswerDtype = 0;

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 0;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA_1);
        // println!("{:?}", input_data);
    }
}
