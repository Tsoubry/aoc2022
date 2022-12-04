type Parsed = ();


fn import_data(data: &str) -> Vec<Parsed> {
    data.lines().map(|line| parse(line)).collect()
}

fn parse(line: &str) -> Parsed {
    todo!()
}

fn answer_part1(data: Vec<Parsed>) -> i64 {
    todo!();
}

// fn answer_part2(data: Vec<Parsed>) -> i64 {
//     todo!();
// }

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    // println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> &'static str {
        todo!();
        r#"
"#
    }

    #[test]
    fn test_answer1() {
        let input_data = import_data(test_data());
        assert_eq!(, answer_part1(input_data));
    }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(test_data());
    //     assert_eq!(, answer_part2(input_data));
    // }

    #[test]
    fn playground() {
        
    }
}
