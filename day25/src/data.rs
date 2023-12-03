pub type AnswerDtype<'a> = &'a str;

pub type Parsed<'a> = &'a str;

pub fn import_data(data: &str) -> Vec<Parsed> {
    data.lines().collect()
}

pub const TEST_DATA_1: &str = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;
pub const TEST_ANSER_1_DEC: i64 = 4890;
pub const TEST_ANSWER_1: AnswerDtype = "2=-1=0";

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = "";

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let _input_data = import_data(TEST_DATA_1);
        // println!("{:?}", input_data);
    }
}
