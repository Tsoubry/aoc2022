use std::ops::Range;

fn import_data(data: &str) -> Vec<(Range<i64>, Range<i64>)> {
    data.lines().map(|line| parse(line)).collect()
}

fn parse(line: &str) -> (Range<i64>, Range<i64>) {
    let output: Vec<_> = line
        .split_terminator(",")
        .map(|section| {
            section
                .split_terminator("-")
                .filter_map(|item| item.parse::<i64>().ok())
                .collect::<Vec<i64>>()
        })
        .map(|section| section[0]..section[1])
        .collect();

    (output[0].clone(), output[1].clone())
}

fn define_inclusive(first: Range<i64>, second: Range<i64>) -> i64 {
    if first.start <= second.start && first.end >= second.end {
        1
    } else if second.start <= first.start && second.end >= first.end {
        1
    } else {
        0
    }
}

fn define_inclusive2(first: &mut Range<i64>, second: &mut Range<i64>) -> i64 {
    first.end += 1;
    second.end += 1;

    for f in first.clone() {
        if second.contains(&f) {
            return 1;
        }
    }

    for s in second {
        if first.contains(&s) {
            return 1;
        }
    }
    0
}

fn answer_part1(data: Vec<(Range<i64>, Range<i64>)>) -> i64 {
    data.into_iter()
        .map(|sections| define_inclusive(sections.0, sections.1))
        .sum()
}

fn answer_part2(data: Vec<(Range<i64>, Range<i64>)>) -> i64 {
    data.into_iter()
        .map(|mut sections| define_inclusive2(&mut sections.0, &mut sections.1))
        .sum()
}

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> &'static str {
        r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#
    }

    #[test]
    fn test_answer1() {
        let input_data = import_data(test_data());
        assert_eq!(2, answer_part1(input_data));
    }

    #[test]
    fn test_answer2() {
        let input_data = import_data(test_data());
        assert_eq!(4, answer_part2(input_data));
    }
}
