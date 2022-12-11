use std::fmt::{Display, write};

use regex::Regex;


// #[derive(new)]
pub struct Monkey {
    pub number: usize,
    pub items_worry: Vec<usize>,
    pub operation: Box<dyn Fn(usize) -> usize>,
    pub test_number: usize,
    pub throw_true: usize,
    pub throw_false: usize,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {}:\n  Starting items: {:?}\n  Test: divisible by {}\n  if true: throw to monkey {}\n  if false: throw to monkey {}", 
        self.number, self.items_worry, self.test_number, self.throw_true, self.throw_false)
    }
}

pub fn import_data(data: &'static str) -> Vec<Monkey> {
    let mut monkeys: Vec<_> = data.split_terminator("\n\n").collect();

    monkeys.iter().map(|monkey_str|parse(&monkey_str)).collect()

}

pub fn parse(monkey_str: &'static str) -> Monkey {
    let re_number = Regex::new(r"^Monkey (\d+).*").unwrap();
    let monkey_number: usize = re_number.captures(&monkey_str).expect("problem with monkey number parsing")
        .get(1).expect("no capture groups for monkey number").as_str().parse().expect("couldn't parse number from string")
        ;

    let re_items = Regex::new(r".*\s\sStarting items:\s(.*)\n").unwrap();
    let unparsed_starting_items = re_items.captures(&monkey_str).expect("problem with parsing starting items")
    .get(1).expect("no capture groups for starting items");

    let starting_items: Vec<usize> = unparsed_starting_items.as_str().split_terminator(", ")
    .map(|item| item.parse::<usize>().expect("couldn't parse number from string for items")).collect();

    let re_operation = Regex::new(r".*\s\sOperation: new = (old|\d+) (\*|\+) (old|\d+)\n").unwrap();
    let caps = re_operation.captures(&monkey_str).expect("problem with operation parsing");
    let first: Option<usize> = caps.get(1).expect("no capture groups for operations").as_str().parse().ok();
    let operator = caps.get(2).expect("no capture groups for operations").as_str();
    let second: Option<usize> = caps.get(3).expect("no capture groups for operations").as_str().parse().ok();
    

    let operation_closure = Box::new(move |old: usize| { 
        match operator {
            "*" => {
                first.unwrap_or(old) * second.unwrap_or(old)
            },
            "+" => {
                first.unwrap_or(old) + second.unwrap_or(old)
            },
            _ => unreachable!()
        }


    });

    let re_test = Regex::new(r".*\s\sTest: divisible by\s(\d+)\n").unwrap();
    let test_number: usize = re_test.captures(&monkey_str).expect("problem with test number parsing")
    .get(1).expect("no capture groups for test number").as_str().parse().expect("couldn't parse test number from string")
    ;

    let re_true = Regex::new(r".*\s\s\s\sIf true: throw to monkey\s(\d+)\n").unwrap();
    let true_number: usize = re_true.captures(&monkey_str).expect("problem with true number parsing")
    .get(1).expect("no capture groups for true number").as_str().parse().expect("couldn't parse true number from string")
    ;

    let re_false = Regex::new(r".*\s\s\s\sIf false: throw to monkey\s(\d+)").unwrap();
    let false_number: usize = re_false.captures(&monkey_str).expect("problem with false number parsing")
    .get(1).expect("no capture groups for false number").as_str().parse().expect("couldn't parse false number from string")
    ;

    Monkey {
        number: monkey_number, 
        items_worry: starting_items, 
        operation: operation_closure, 
        test_number, 
        throw_true: true_number, 
        throw_false: false_number,
    }

}

pub const TEST_DATA: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA);

        input_data
            .into_iter()
            .for_each(|monkey| println!("{}", monkey));
    }

    #[test]
    fn test_closures() {
        let input_data = import_data(TEST_DATA);

        let monkey = input_data.first().unwrap();

        let c = monkey.operation.as_ref();

        assert_eq!(1501, monkey.operation.as_ref()(*monkey.items_worry.first().unwrap()))

    }
}
