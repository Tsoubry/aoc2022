use std::collections::HashMap;

fn import_data(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|line| line.chars().collect()).collect()
}

fn letter_values(priorities: &HashMap<char, usize>, c: char) -> i64 {
    *priorities.get(&c).unwrap() as i64
}

fn get_priorities() -> HashMap<char, usize> {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .map(|x| (x.1, x.0 + 1))
        .collect()
}

fn find_double(comp1: &[char], comp2: &[char]) -> Option<char> {
    for c in comp1 {
        if comp2.contains(c) {
            return Some(*c);
        }
    }
    None
}

fn find_group_char(g1: &[char], g2: &[char], g3: &[char]) -> Option<char> {
    for c in g1 {
        if g2.contains(c) && g3.contains(c) {
            return Some(*c);
        }
    }
    None
}

fn answer1(data: Vec<Vec<char>>, priorities: HashMap<char, usize>) -> i64 {
    data.into_iter()
        .map(|rucksack| {
            let compartment_size = rucksack.len() / 2;
            let (comp1, comp2) = rucksack.split_at(compartment_size);
            find_double(comp1, comp2)
        })
        .map(|double_item| letter_values(&priorities, double_item.unwrap()))
        .sum()
}

fn answer2(data: Vec<Vec<char>>, priorities: HashMap<char, usize>) -> i64 {
    data.chunks(3)
        .filter_map(|group| find_group_char(&group[0], &group[1], &group[2]))
        .map(|x| letter_values(&priorities, x))
        .sum()
}

fn main() {
    let priorities = get_priorities();

    let input_data = import_data(include_str!("../input.txt"));

    println!(
        "Answer of part 1 is: {}",
        answer1(input_data.clone(), priorities.clone())
    );
    println!(
        "Answer of part 2 is: {}",
        answer2(input_data.clone(), priorities)
    );
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_answer1() {
        let test_data = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

        let priorities = get_priorities();

        let input_data = import_data(test_data);
        assert_eq!(157, answer1(input_data, priorities));
    }

    #[test]
    fn test_answer2() {
        let priorities = get_priorities();

        let test_data = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

        let input_data = import_data(test_data);

        assert_eq!(70, answer2(input_data, priorities));
    }
}
