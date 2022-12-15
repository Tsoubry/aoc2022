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

        // println!("y range: {}", y_range);

        if y_range > 0 {
            let x_start = sensor.x - y_range;
            let x_end = sensor.x + y_range + 1;

            // println!("x start: {}, x end: {}", x_start, x_end);

            for x_point in min(x_start, x_end)..max(x_start, x_end) {
                places_marked.insert(x_point);
            }
        }

        // let mut iter = min(sensor.y, beacon.y)..(max(sensor.y, beacon.y) + 1);

        // if iter.contains(&y_level) {

        //     let diff = max(sensor.y, beacon.y) - y_level;

        // }
    }

    let mut v = places_marked.iter().collect::<Vec<_>>();
    v.sort();

    // println!("hashset: {:?}", v);
    // println!("remove beacons: {:?}", &beacon_locations);

    places_marked.len() - beacon_locations.len()
}

// fn answer_part2(data: Vec<Parsed>) -> i64 {

// }

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

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(, answer_part2(input_data));
    // }

    #[test]
    fn playground() {}
}
