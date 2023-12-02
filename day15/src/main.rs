pub mod data;

use std::cmp::{max, min};
use std::collections::HashSet;

#[macro_use]
extern crate derive_new;

use crate::data::*;

fn answer_part1(data: Vec<(Coordinate, Coordinate)>, y_level: isize) -> usize {
    let mut places_marked: HashSet<isize> = HashSet::new(); // x
    let mut beacon_locations: HashSet<isize> = HashSet::new();

    for (sensor, beacon) in data {
        if beacon.y == y_level {
            beacon_locations.insert(beacon.x);
        }

        let sensor_range = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();

        let y_range = sensor_range - (sensor.y - y_level).abs();

        if y_range > 0 {
            let x_start = sensor.x - y_range;
            let x_end = sensor.x + y_range + 1;

            for x_point in min(x_start, x_end)..max(x_start, x_end) {
                places_marked.insert(x_point);
            }
        }
    }

    let mut v = places_marked.iter().collect::<Vec<_>>();
    v.sort();

    places_marked.len() - beacon_locations.len()
}

fn answer_part2(data: Vec<(Coordinate, Coordinate)>, x_max: usize, y_max: usize) -> usize {
    // sensors

    // start with range of sensors

    for (sensor, beacon) in data {}

    let x_min = 0;
    let y_min = 0;

    // beacon between 0 and
    // 4_000_000

    // tuning frequency: x * 4_000_000 + y

    let distress_beacon_x = 20;
    let distress_beacon_y = 10;

    distress_beacon_x * 4_000_000 + distress_beacon_y
}

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    let y_level = 2_000_000;

    println!(
        "Answer of part 1 is: {}",
        answer_part1(input_data.clone(), y_level)
    );
    // println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::data::TEST_DATA;

    #[test]
    fn test_answer1() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(26, answer_part1(input_data, 10));
    }

    #[test]
    fn test_answer2() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(56000011, answer_part2(input_data, 20, 20));
    }

    #[test]
    fn playground() {}
}
