pub mod data;

// use itertools::Itertools;

use std::collections::VecDeque;

use crate::data::*;

fn find_xth_number(list: &VecDeque<isize>, n: usize) -> isize {
    let zero_index = list
        .iter()
        .enumerate()
        .find(|x| x.1 == &0)
        .expect("no zero in list")
        .0;
    let item_index = (zero_index + n) % list.len();
    *list.get(item_index).unwrap()
}

fn answer_part1(data: Vec<isize>) -> isize {
    let mut list = VecDeque::from_iter(data.clone().into_iter().enumerate());

    let list_size = data.len();

    for i in 0..list_size {
        // println!("{:?}", &list.iter().map(|x| x.1).collect::<Vec<_>>());
        // println!("{:?}", &list);

        let (index, _) = list
            .iter()
            .enumerate()
            .find(|item| item.1 .0 == i)
            .expect("Item with index not found")
            .clone();

        let item = &mut list
            .remove(index)
            .expect("item couldn't be removed from list");

        // println!("item: {:?}", item);
        // println!("after remove {:?}", &list);

        let new_index = index as isize + item.1;

        let modulo = new_index % list_size as isize;

        let final_index = match new_index.signum() {
            -1 => list_size as isize + modulo - 1,
            0 => list_size as isize - 1,
            _ => {
                // positive

                if new_index / list_size as isize >= 1 {
                    modulo + 1
                } else {
                    modulo
                }
            }
        };

        // println!("final_index: {}", final_index);

        let _ = &mut list.insert(final_index as usize, *item);
        // println!("after insert {:?}", &list);
        // println!("{:?}", &list.iter().map(|x| x.1).collect::<Vec<_>>());
    }

    let item_only = list.into_iter().map(|x| x.1).collect();

    find_xth_number(&item_only, 1000)
        + find_xth_number(&item_only, 2000)
        + find_xth_number(&item_only, 3000)
}

// fn answer_part2(data: Vec<Parsed>) -> i64 {

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
        assert_eq!(3, answer_part1(input_data));
    }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(, answer_part2(input_data));
    // }

    #[test]
    fn test_nth_function() {
        let t = VecDeque::from_iter(vec![1, 2, -3, 4, 0, 3, -2].into_iter());

        assert_eq!(find_xth_number(&t, 1000), 4);
        assert_eq!(find_xth_number(&t, 2000), -3);
        assert_eq!(find_xth_number(&t, 3000), 2);
    }
}
