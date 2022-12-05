use std::collections::{HashMap, VecDeque};

type Parsed = (usize, usize, usize);

type Stack = VecDeque<char>;

fn import_data(data: &str) -> (HashMap<usize, Stack>, Vec<Parsed>) {
    let mut lines = data.lines();

    let mut stacks: HashMap<usize, Stack> = HashMap::new();

    for line in &mut lines {
        let crates: Vec<_> = line
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|chunk| chunk[1])
            .collect();

        for crate_number in 0..crates.len() {
            if crates[crate_number].is_ascii_alphabetic() {
                stacks
                    .entry(crate_number + 1)
                    .and_modify(|deque| deque.push_front(crates[crate_number]))
                    .or_insert(VecDeque::from_iter([crates[crate_number]]));
            } else {
                continue;
            }
        }

        if !line.contains("[") {
            break;
        }
    }

    lines.next(); // skip empty line

    let crane_orders: Vec<Parsed> = lines.into_iter().map(|line| parse(line)).collect();

    (stacks, crane_orders)
}

fn parse(line: &str) -> Parsed {
    let numbers: Vec<_> = line
        .replace("move ", "")
        .replace(" from ", ",")
        .replace(" to ", ",")
        .split_terminator(",")
        .map(|number| number.parse::<usize>().unwrap())
        .collect();

    (numbers[0], numbers[1], numbers[2])
}

fn answer_part1(stacks: &mut HashMap<usize, Stack>, operations: Vec<Parsed>) -> String {
    for (crates_to_move, from, to) in operations {
        for _ in 0..crates_to_move {
            let extracted_crate = stacks.get_mut(&from).unwrap().pop_back().unwrap();
            stacks
                .entry(to)
                .and_modify(|deque| deque.push_back(extracted_crate));
        }
    }

    let mut solution: Vec<char> = vec![];

    for n in 0..stacks.len() {
        solution.push(*stacks.get(&(n + 1)).unwrap().back().unwrap())
    }

    String::from_iter(solution)
}

fn answer_part2(stacks: &mut HashMap<usize, Stack>, operations: Vec<Parsed>) -> String {
    for (crates_to_move, from, to) in operations {
        let stack = stacks.get_mut(&from).unwrap();
        let start_at = stack.len() - crates_to_move;
        let crates_to_stack: Vec<char> = stack.drain(start_at..).collect();

        for item in crates_to_stack {
            stacks.entry(to).and_modify(|deque| deque.push_back(item));
        }
    }

    let mut solution: Vec<char> = vec![];

    for n in 0..stacks.len() {
        solution.push(*stacks.get(&(n + 1)).unwrap().back().unwrap())
    }

    String::from_iter(solution)
}

fn main() {
    let (mut stacks, operations) = import_data(include_str!("../input.txt"));

    println!(
        "Answer of part 1 is: {}",
        answer_part1(&mut stacks.clone(), operations.clone())
    );
    println!(
        "Answer of part 2 is: {}",
        answer_part2(&mut stacks, operations)
    );
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> &'static str {
        r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#
    }

    #[test]
    fn test_answer1() {
        let (mut stacks, operations) = import_data(test_data());

        assert_eq!("CMZ".to_string(), answer_part1(&mut stacks, operations));
    }

    #[test]
    fn test_answer2() {
        let (mut stacks, operations) = import_data(test_data());

        assert_eq!("MCD".to_string(), answer_part2(&mut stacks, operations));
    }

    #[test]
    fn playground() {}
}
