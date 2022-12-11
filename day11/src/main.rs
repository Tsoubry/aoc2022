pub mod data;

use std::collections::HashMap;

use crate::data::*;

fn add_items_to_monkey(monkeys: &mut Vec<Monkey>, monkey_nr: usize, items: Vec<usize>) {
    let monkey = monkeys
        .iter_mut()
        .find(|mon| mon.number == monkey_nr)
        .expect("Monkey not found");

    monkey.items_worry.extend(items.iter());
}

fn answer_part1(data: Vec<Monkey>) -> usize {
    let mut monkeys = data;

    let mut inspections: HashMap<usize, usize> = HashMap::new();

    for _round in 0..20 {
        let mut ledger: HashMap<usize, Vec<usize>> = HashMap::new();

        for monkey_nr in 0..monkeys.len() {
            let mut item_to_transfer: Option<usize>;
            let mut transfer_monkey: usize;

            let monkey = monkeys
                .iter_mut()
                .find(|mon| mon.number == monkey_nr)
                .expect("Monkey not found");

            for _ in 0..monkey.items_worry.len() {
                item_to_transfer = monkey.items_worry.pop_front();

                if let Some(item) = item_to_transfer {
                    inspections
                        .entry(monkey.number)
                        .and_modify(|inspect| *inspect += 1)
                        .or_insert(1);

                    let new_worry_level = monkey.operation.as_ref()(item);
                    let level_after_bored: usize =
                        ((new_worry_level as f64) / 3.0).floor() as usize;

                    if level_after_bored % monkey.test_number == 0 {
                        transfer_monkey = monkey.throw_true;
                    } else {
                        transfer_monkey = monkey.throw_false;
                    }

                    ledger
                        .entry(transfer_monkey)
                        .and_modify(|items| items.push(level_after_bored))
                        .or_insert(vec![level_after_bored]);
                }
            }

            for (key, value) in ledger.drain() {
                add_items_to_monkey(&mut monkeys, key, value);
            }

            // println!("After round {}:", _round + 1);
            // monkeys.iter().for_each(|monkey| println!("Monkey {}: {:?}", monkey.number, monkey.items_worry));
        }
    }

    let mut all_inspections: Vec<usize> = inspections.into_iter().map(|(_, value)| value).collect();
    all_inspections.sort();
    all_inspections.reverse();

    all_inspections.first().unwrap() * all_inspections.get(1).unwrap()
}

// fn answer_part2(data: Vec<Parsed>) -> i64 {

// }

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data));
    // println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::data::TEST_DATA;

    #[test]
    fn test_answer1() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(10605, answer_part1(input_data));
    }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(, answer_part2(input_data));
    // }

    #[test]
    fn playground() {}
}
