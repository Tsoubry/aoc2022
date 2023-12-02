use derive_new::new;
use regex::Regex;

pub type AnswerDtype = u32;

const PATTERN: &str = r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.";

#[derive(new, Debug, Copy, Clone)]
pub struct Money {
    pub ore: u32,
    pub clay: u32,
    pub obsidian: u32,
}

#[derive(new, Debug, Copy, Clone)]
pub struct Blueprint {
    pub number: u32,
    pub ore_cost: Money,
    pub clay_cost: Money,
    pub obsidian_cost: Money,
    pub geode_cost: Money,
}

pub fn import_data(data: &str) -> Vec<Blueprint> {
    let re = Regex::new(PATTERN).unwrap();

    data.lines().map(|line| parse(line, &re)).collect()
}

fn parse_str_to_num(s: &str) -> u32 {
    s.parse().expect("couldn't parse str to u32")
}

pub fn parse(line: &str, re: &Regex) -> Blueprint {
    let (_, [blueprint_num, ore_ore, clay_ore, obs_ore, obs_clay, geo_ore, geo_obs]) = re
        .captures(line)
        .expect("capture with regex failed")
        .extract();

    Blueprint::new(
        parse_str_to_num(blueprint_num),
        Money::new(parse_str_to_num(ore_ore), 0, 0),
        Money::new(parse_str_to_num(clay_ore), 0, 0),
        Money::new(parse_str_to_num(obs_ore), parse_str_to_num(obs_clay), 0),
        Money::new(parse_str_to_num(geo_ore), 0, parse_str_to_num(geo_obs)),
    )
}

pub const TEST_DATA_1: &str = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;
pub const TEST_ANSWER_1: AnswerDtype = 33;

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 0;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA_1);
        println!("{:?}", input_data);
    }
}
