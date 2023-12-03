pub mod data;

pub use crate::data::*;

pub const PENTA_TABLE: [i64; 28] = [
    1,
    5,
    25,
    125,
    625,
    3125,
    15625,
    78125,
    390625,
    1953125,
    9765625,
    48828125,
    244140625,
    1220703125,
    6103515625,
    30517578125,
    152587890625,
    762939453125,
    3814697265625,
    19073486328125,
    95367431640625,
    476837158203125,
    2384185791015625,
    11920928955078125,
    59604644775390625,
    298023223876953125,
    1490116119384765625,
    7450580596923828125,
];

pub fn convert_from_snafu(input: &str) -> i64 {
    let mut num: i64 = 0;

    for (idx, c) in input.chars().rev().enumerate() {
        num += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Invalid character"),
        } * PENTA_TABLE[idx];
    }

    num
}

pub fn convert_to_snafu(mut num: i64) -> String {
    let mut snafu = String::new();

    while num != 0 {
        let digit = match (num + 2) % 5 - 2 {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        };
        snafu.push(digit);
        num = (num + 2) / 5;
    }

    snafu.chars().rev().collect()
}

pub fn answer_part1(data: Vec<Parsed>) -> String {
    let decimal = data.iter().map(|x| convert_from_snafu(x)).sum::<i64>();

    convert_to_snafu(decimal)
}

// pub fn answer_part2(data: Vec<Parsed>) -> AnswerDtype {
//     todo!()
// }
