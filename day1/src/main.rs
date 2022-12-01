fn parse_data(data: &str) -> Vec<Option<i64>> {
    data.lines().map(|line| line.parse::<i64>().ok()).collect()
}

fn answer1(data: Vec<Option<i64>>) -> i64 {
    let mut max_item = 0;

    let mut current_item = 0;

    for item in data {
        match item {
            Some(x) => current_item += x,
            None => {
                if current_item > max_item {
                    max_item = current_item;
                }
                current_item = 0;
            }
        }
    }

    max_item
}

fn answer2(data: Vec<Option<i64>>) -> i64 {
    let mut all_items: Vec<i64> = vec![];

    let mut current_item = 0;

    for item in data {
        match item {
            Some(x) => current_item += x,
            None => {
                all_items.push(current_item);
                current_item = 0;
            }
        }
    }

    all_items.sort();
    all_items.reverse();

    all_items[0..3].iter().sum()
}

fn main() {
    let input_data = parse_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer1(input_data.clone()));
    println!("Answer of part 2 is: {}", answer2(input_data.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer2() {
        let test_data = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

"#;

        let parsed_data = parse_data(test_data);
        println!("{:?}", &parsed_data);
        assert_eq!(45000, answer2(parsed_data));
    }
}
