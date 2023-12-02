pub mod data;

pub use crate::data::*;

#[derive(Debug, Copy, Clone)]
struct Resources {
    money: Money,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            money: Money::new(0, 0, 0),
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

fn calculate_quality_level(blueprint_id: u32, max_geodes: u32) -> u32 {
    blueprint_id * max_geodes
}

fn make_choice() {

    

}

fn find_optimal_resources(blueprint: &Blueprint, mut resources: Resources, max_minutes: u32) -> Resources {

    let mut best_result = Resources::default();

    for minute in 0..max_minutes {
        
    }


    best_result
}

pub fn answer_part1(data: Vec<Blueprint>) -> AnswerDtype {
    let minutes = 24;
    let resources = Resources::default();

    data.into_iter()
        .map(|blueprint| {
            let optimal_resources = find_optimal_resources(&blueprint, resources.clone(), minutes);
            (blueprint, optimal_resources)
        })
        .map(|(blueprint, resources)| {
            calculate_quality_level(blueprint.number, resources.geode_robots)
        })
        .sum()
}

// pub fn answer_part2(data: Vec<Parsed>) -> AnswerDtype {
//     todo!()
// }
