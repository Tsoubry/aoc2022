pub mod data;

use std::collections::HashMap;

use pathfinding::prelude::astar;

#[macro_use]
extern crate derive_new;

use crate::data::*;

#[derive(new, Debug, Clone, Hash, Eq)]
pub struct ValvePath {
    pub current: String,
    pub flow_rate: isize,
    pub tunnels: Vec<String>,
    pub minutes_left: usize,
    pub opened_valves: Vec<String>,
}

impl PartialEq for ValvePath {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current && self.minutes_left == other.minutes_left
    }
}

impl ValvePath {
    fn from_valve(valve: Valve, minutes_left: usize, opened_valves: Vec<String>) -> Self {
        Self {
            current: valve.name,
            flow_rate: valve.flow_rate,
            tunnels: valve.tunnels,
            minutes_left,
            opened_valves,
        }
    }
}

fn open_valve(mut valve: ValvePath) -> (ValvePath, isize) {
    let minutes_flowing = valve.minutes_left - 1;

    let add_score = valve.flow_rate.checked_neg().unwrap() * minutes_flowing as isize;

    valve.minutes_left -= 2;
    valve.opened_valves.push(valve.current.clone());

    (valve, add_score)
}

fn move_to_next(mut valve: ValvePath) -> (ValvePath, isize) {
    valve.minutes_left -= 1;

    (valve, 0)
}

fn heuristic(current_path: &ValvePath, map: &HashMap<String, Valve>) -> isize {

    -1653
}



fn successors(valve_path: &ValvePath, map: &HashMap<String, Valve>) -> Vec<(ValvePath, isize)> {
    let tunnel_iter = valve_path.tunnels.iter().map(|tunnel| {
        ValvePath::from_valve(
            map.get(tunnel).expect("tunnel not found in map").clone(),
            valve_path.minutes_left,
            valve_path.opened_valves.clone(),
        )
    });

    let mut opened_valves_option: Vec<_> = tunnel_iter
        .clone()
        .filter(|valve| !valve_path.opened_valves.contains(&valve.current))
        .map(|valve| open_valve(valve))
        .collect();

    let move_to_next_option: Vec<_> = tunnel_iter.map(|valve| move_to_next(valve)).collect();

    opened_valves_option.extend(move_to_next_option);

    opened_valves_option
}

fn answer_part1(data: HashMap<String, Valve>) -> isize {
    let start_valve =
        ValvePath::from_valve(data.get(&"AA".to_string()).unwrap().clone(), 30, vec![]);

    let result = astar(
        &start_valve,
        |valve| successors(valve, &data),
        |valve| heuristic(valve, &data),
        |valve| valve.minutes_left == 0, // || valve.minutes_left == 1
    );

    // println!("{:?}", &result.0.iter().map(|res| res.1.1));

    print!("{:?}", &result);

    // let best_cost: isize = result.0.into_iter().map(|res| res.1.1).min().unwrap();

    result.unwrap().1


    // best_cost
}

// fn answer_part2(data: HashMap<String, Valve>) -> usize {

// }

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    // println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::data::TEST_DATA;

    #[test]
    fn test_answer1() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(-1651, answer_part1(input_data));
    }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(, answer_part2(input_data));
    // }

    #[test]
    fn playground() {}
}
