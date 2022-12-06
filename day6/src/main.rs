use std::collections::HashSet;
use std::collections::VecDeque;

fn import_data(data: &str) -> String {
    data.lines().last().unwrap().to_string()
}

// Could have done it with windows, but I didn't think of it in time
fn solve_for(data: String, n: usize) -> i64 {
    let mut last_four: VecDeque<char> = VecDeque::new();

    let chars = data.chars();

    for (char_number, character) in chars.enumerate() {
        if last_four.len() == n {
            last_four.pop_front().unwrap();
        };

        last_four.push_back(character);

        let uniques: HashSet<_> = last_four.clone().into_iter().collect();

        if uniques.len() == n {
            return char_number as i64 + 1;
        }
    }

    0
}

fn answer_part1(data: String) -> i64 {
    solve_for(data, 4)
}

fn answer_part2(data: String) -> i64 {
    solve_for(data, 14)
}

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> Vec<String> {
        vec![
            "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(),
            "nppdvjthqldpwncqszvftbrmjlhg".to_string(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(),
        ]
    }

    fn test_data2() -> Vec<String> {
        vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(),
            "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(),
            "nppdvjthqldpwncqszvftbrmjlhg".to_string(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(),
        ]
    }

    #[test]
    fn test_answer1() {
        let test_data = test_data();

        let expected = vec![5, 6, 10, 11];

        test_data
            .into_iter()
            .zip(expected.into_iter())
            .for_each(|item| assert_eq!(answer_part1(item.0), item.1))
    }

    #[test]
    fn test_answer2() {
        let test_data = test_data2();

        let expected = vec![19, 23, 23, 29, 26];

        test_data
            .into_iter()
            .zip(expected.into_iter())
            .for_each(|item| assert_eq!(answer_part2(item.0), item.1))
    }
}
