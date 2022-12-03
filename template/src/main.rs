fn import_data(data: &str) -> Vec<_> {
    data.lines().map(|line| line.chars().collect()).collect()
}

fn answer_part1(data: Vec<_>) -> i64 {
    todo!();
}

fn answer_part2(data: Vec<_>) -> i64 {
    todo!();
}

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> &str {
        r#"
"#;
    }

    #[test]
    fn test_answer1() {
        let input_data = import_data(test_data());
        assert_eq!(, answer1(input_data, priorities));
    }

    #[test]
    fn test_answer2() {
        let input_data = import_data(test_data());
        assert_eq!(, answer2(input_data, priorities));
    }

    #[test]
    fn playground() {
        
    }
}
