pub mod data;

use std::collections::HashMap;

use pathfinding::directed::dfs::dfs;

#[macro_use]
extern crate derive_new;

use crate::data::*;

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub struct Score {
    pub minutes: usize,
    pub total_flow: usize,
    pub opened_valves: Vec<String>,
}

fn open_valve(valve: &Valve, score: &Score) -> Score {
    let minutes_flowing = score.minutes.checked_sub(1).unwrap_or(0);

    let new_flow = valve.flow_rate * minutes_flowing;

    let mut opened_valves = score.opened_valves.clone();
    opened_valves.push(valve.name.clone());

    Score::new(minutes_flowing.checked_sub(1).unwrap_or(0), score.total_flow + new_flow, opened_valves)
}

fn move_to_next(valve: &Valve, score: &Score) -> Score {
    Score::new(score.minutes.checked_sub(1).unwrap_or(0), score.total_flow, score.opened_valves.clone())
}

fn successors(
    (valve, score): (&Valve, &Score),
    map: &HashMap<String, Valve>,
) -> Vec<(Valve, Score)> {
    let tunnel_iter = valve
        .tunnels
        .iter()
        .map(|tunnel| (map.get(tunnel).expect("tunnel not found in map").clone()));

    let mut opened_valves_option: Vec<_> = tunnel_iter
        .clone()
        .filter(|valve| !score.opened_valves.contains(&valve.name))
        .map(|valve| {
            let updated_score = open_valve(&valve, score);
            (valve, updated_score)
        })
        .collect();

    let move_to_next_option: Vec<_> = tunnel_iter
        .map(|valve| {
            let updated_score = move_to_next(&valve, score);
            (valve, updated_score)
        })
        .collect();

    opened_valves_option.extend(move_to_next_option);

    opened_valves_option
}

fn calculate_total_flow(path: Vec<(Valve, Score)>) -> usize {
    path.last().unwrap().1.total_flow
}

fn answer_part1(data: HashMap<String, Valve>) -> usize {
    let start_valve = data.get(&"AA".to_string()).unwrap().clone(); // should be in the map

    let result = dfs(
        (start_valve, Score::new(30, 0, vec![])),
        |(valve, score)| successors((valve, score), &data),
        |(_, score)| score.minutes == 0 && score.total_flow == 1651,
    );

    // println!("{:?}", &result);

    calculate_total_flow(result.unwrap())
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
        assert_eq!(1651, answer_part1(input_data));
    }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(, answer_part2(input_data));
    // }

    #[test]
    fn playground() {}
}
