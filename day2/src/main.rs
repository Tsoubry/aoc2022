use std::cmp::Ordering;

#[derive(PartialEq, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Clone)]
enum Strategy {
    Lose,
    Draw,
    Win,
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self < other {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }

    fn lt(&self, other: &Self) -> bool {
        (self == &RPS::Rock && other == &RPS::Paper)
            || (self == &RPS::Paper && other == &RPS::Scissors)
            || (self == &RPS::Scissors && other == &RPS::Rock)
    }
}

impl RPS {
    fn parse(text: &str) -> (RPS, RPS) {
        let mut chars = text.chars();
        let opponent = match chars.next().unwrap() {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => unreachable!(),
        };
        let you = match chars.last().unwrap() {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => unreachable!(),
        };
        (opponent, you)
    }

    fn shape_score(&self) -> i64 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn total_score((opponent, you): (RPS, RPS)) -> (i64, i64) {
        let win_score = if opponent == you {
            (3, 3)
        } else if opponent > you {
            (6, 0)
        } else {
            (0, 6)
        };

        (
            win_score.0 + opponent.shape_score(),
            win_score.1 + you.shape_score(),
        )
    }

    fn parse2(text: &str) -> (RPS, Strategy) {
        let mut chars = text.chars();
        let opponent = match chars.next().unwrap() {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => unreachable!(),
        };
        let you = match chars.last().unwrap() {
            'X' => Strategy::Lose,
            'Y' => Strategy::Draw,
            'Z' => Strategy::Win,
            _ => unreachable!(),
        };
        (opponent, you)
    }

    fn total_score2((opponent, your_strategy): (RPS, Strategy)) -> (i64, i64) {
        let win_rps = if opponent == RPS::Rock {
            RPS::Paper
        } else if opponent == RPS::Paper {
            RPS::Scissors
        } else {
            RPS::Rock
        };

        let lose_rps = if opponent == RPS::Rock {
            RPS::Scissors
        } else if opponent == RPS::Paper {
            RPS::Rock
        } else {
            RPS::Paper
        };

        match your_strategy {
            Strategy::Draw => RPS::total_score((opponent.clone(), opponent)),
            Strategy::Lose => RPS::total_score((opponent, lose_rps)),
            Strategy::Win => RPS::total_score((opponent, win_rps)),
        }
    }
}

fn parse_data(data: &str) -> Vec<(RPS, RPS)> {
    data.lines().map(|line| RPS::parse(line)).collect()
}

fn parse_data2(data: &str) -> Vec<(RPS, Strategy)> {
    data.lines().map(|line| RPS::parse2(line)).collect()
}

fn answer1(data: Vec<(RPS, RPS)>) -> i64 {
    data.into_iter()
        .map(|game| RPS::total_score(game))
        .map(|result| result.1)
        .sum()
}

fn answer2(data: Vec<(RPS, Strategy)>) -> i64 {
    data.into_iter()
        .map(|game| RPS::total_score2(game))
        .map(|result| result.1)
        .sum()
}

fn main() {
    let input_data = parse_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer1(input_data));

    let input_data2 = parse_data2(include_str!("../input.txt"));

    println!("Answer of part 2 is: {}", answer2(input_data2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer1() {
        let test_data = r#"A Y
B X
C Z
"#;

        let parsed_data = parse_data(test_data);
        assert_eq!(15, answer1(parsed_data));
    }

    #[test]
    fn test_answer2() {
        let test_data = r#"A Y
B X
C Z
"#;

        let parsed_data = parse_data2(test_data);
        assert_eq!(12, answer2(parsed_data));
    }
}
