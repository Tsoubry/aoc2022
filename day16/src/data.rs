use std::collections::HashMap;

use regex::Regex;

#[derive(new, Debug, Clone)]
pub struct Valve {
    pub name: String,
    pub flow_rate: usize,
    pub tunnels: Vec<String>,
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Valve {}

pub fn import_data(data: &str) -> HashMap<String, Valve> {
    data.lines().map(|line| parse(line)).collect()
}

pub fn parse(line: &str) -> (String, Valve) {
    let re =
        Regex::new(r"Valve ([A-Z]+).+rate=(\d+); tunnels? leads? to valves? ([,A-Z ]+)").unwrap();

    let caps = re.captures(&line).expect("no capture groups");

    let name = caps
        .get(1)
        .expect("first capture group")
        .as_str()
        .to_string();
    let flow: usize = caps
        .get(2)
        .expect("second capture group")
        .as_str()
        .parse()
        .expect("problem with parsing flow rate");
    let tunnels: Vec<_> = caps
        .get(3)
        .expect("third capture group")
        .as_str()
        .split_terminator(", ")
        .map(|valve| valve.to_string())
        .collect();

    (name.clone(), Valve::new(name, flow, tunnels))
}

pub const TEST_DATA: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA);
        // println!("{:?}", input_data);
    }
}
