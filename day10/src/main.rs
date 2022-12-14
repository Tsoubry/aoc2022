#![allow(non_camel_case_types)]
pub mod test_data;

use strum_macros::EnumString;

use std::fmt::Write;
use std::str::FromStr;

const SIGNAL_CYCLE: [usize; 6] = [20, 60, 100, 140, 180, 220];
const CRT_CYCLE: [usize; 6] = [40, 80, 120, 160, 200, 240];

#[derive(Debug, Copy, Clone, EnumString, PartialEq)]
enum Op {
    noop,
    addx,
}

type Instruction = (Op, Option<isize>);

fn import_data(data: &str) -> Vec<Instruction> {
    data.lines().map(|line| parse(line)).collect()
}

fn parse(line: &str) -> Instruction {
    let mut splitted = line.split_whitespace();
    let operation = Op::from_str(splitted.next().unwrap()).unwrap();
    let amount: Option<isize> = match splitted.next() {
        Some(s) => s.parse().ok(),
        None => None,
    };

    (operation, amount)
}

fn answer_part1(data: Vec<Instruction>) -> isize {
    let mut register: isize = 1;
    let mut cycle: usize = 0;
    let mut signal_strength: isize = 0;

    for (operation, value) in data {
        // instructions
        let cycles_left = match operation {
            Op::noop => 1,
            Op::addx => 2,
        };

        // computation
        for _ in 0..cycles_left {
            cycle += 1;

            if SIGNAL_CYCLE.contains(&cycle) {
                let signal_strength_to_add = cycle as isize * register;
                signal_strength += signal_strength_to_add;
            }
        }

        // finishing execution
        if let Some(instruction_value) = value {
            register += instruction_value;
        };
    }

    signal_strength
}

fn answer_part2(data: Vec<Instruction>) -> String {
    let mut crt = String::new();

    let mut register: isize = 1;
    let mut cycle: usize = 0;
    let mut cursor: isize = 0;

    for (operation, value) in data {
        // instructions
        let cycles_left = match operation {
            Op::noop => 1,
            Op::addx => 2,
        };

        // computation
        for _ in 0..cycles_left {
            cycle += 1;

            if [cursor - 1, cursor, cursor + 1].contains(&register) {
                write!(crt, "#").unwrap();
            } else {
                write!(crt, ".").unwrap();
            }

            cursor += 1;

            if CRT_CYCLE.contains(&cycle) {
                write!(crt, "\n").unwrap();
                cursor = 0;
            }
        }

        // finishing execution
        if let Some(instruction_value) = value {
            register += instruction_value;
        };
    }

    crt
}

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    println!("Answer of part 2 is:\n{}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::test_data::*;

    #[test]
    fn test_parsing() {
        let _input_data = import_data(TEST_DATA);
        // println!("{:?}", _input_data);
    }

    #[test]
    fn test_answer1() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(13140, answer_part1(input_data));
    }

    #[test]
    fn test_answer2() {
        let input_data = import_data(TEST_DATA);
        let answer = answer_part2(input_data);
        assert_eq!(RESULT_CRT, answer);
    }
}
