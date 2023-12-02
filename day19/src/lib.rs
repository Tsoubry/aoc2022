use std::vec;

use par_dfs::sync::{Dfs, Node, NodeIter};

pub mod data;

pub use crate::data::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Resources {
    blueprint: Blueprint,
    money: Money,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    geodes: u32,
}

impl Resources {
    fn new(blueprint: Blueprint) -> Self {
        Self {
            blueprint,
            money: Money::new(0, 0, 0),
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            geodes: 0,
        }
    }

    fn advance(&mut self) {
        self.money.ore += self.ore_robots;
        self.money.clay += self.clay_robots;
        self.money.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }
}

impl Node for Resources {
    type Error = std::convert::Infallible;

    fn children(&self, _depth: usize) -> NodeIter<Self, Self::Error> {
        let mut options: Vec<Self> = vec![];

        // don't do anything

        let mut resources = self.clone();
        resources.advance();

        options.push(resources);

        // robot buying

        // Ore robot
        if self.money.ore >= self.blueprint.ore_cost.ore {
            let mut resources = self.clone();
            resources.money.ore -= self.blueprint.ore_cost.ore;
            resources.advance();
            resources.ore_robots += 1;
            options.push(resources);
        }

        // Clay robot
        if self.money.ore >= self.blueprint.clay_cost.ore {
            let mut resources = self.clone();
            resources.money.ore -= self.blueprint.clay_cost.ore;
            resources.advance();
            resources.clay_robots += 1;
            options.push(resources);
        }

        // Obsidian robot
        if self.money.ore >= self.blueprint.obsidian_cost.ore
            && self.money.clay >= self.blueprint.obsidian_cost.clay
        {
            let mut resources = self.clone();
            resources.money.ore -= self.blueprint.obsidian_cost.ore;
            resources.money.clay -= self.blueprint.obsidian_cost.clay;
            resources.advance();
            resources.obsidian_robots += 1;
            options.push(resources);
        }

        // Geode robot
        if self.money.ore >= self.blueprint.geode_cost.ore
            && self.money.obsidian >= self.blueprint.geode_cost.obsidian
        {
            let mut resources = self.clone();
            resources.money.ore -= self.blueprint.geode_cost.ore;
            resources.money.obsidian -= self.blueprint.geode_cost.obsidian;
            resources.advance();
            resources.geode_robots += 1;
            options.push(resources);
        }

        let nodes = options.into_iter().map(Result::Ok);

        Ok(Box::new(nodes))
    }
}

fn calculate_quality_level(blueprint_id: u32, max_geodes: u32) -> u32 {
    blueprint_id * max_geodes
}

fn find_optimal_resources(resources: Resources, max_minutes: u32) -> Resources {
    let dfs = Dfs::<Resources>::new(resources, Some(max_minutes as usize), false);

    let output = dfs
        .map(|result| result)
        .collect::<Result<Vec<_>, _>>()
        .expect("error in dfs");

    output
        .into_iter()
        .max_by_key(|resources| resources.geode_robots)
        .expect("error when maximizing geode robots")
}

pub fn answer_part1(data: Vec<Blueprint>) -> AnswerDtype {
    let minutes = 24; // check this based on iterations dfs

    data.into_iter()
        .map(|blueprint| {
            let resources = Resources::new(blueprint);
            let optimal_resources = find_optimal_resources(resources, minutes);
            optimal_resources
        })
        .map(|resources| {
            calculate_quality_level(resources.blueprint.number, resources.geodes)
        })
        .sum()
}

// pub fn answer_part2(data: Vec<Parsed>) -> AnswerDtype {
//     todo!()
// }
