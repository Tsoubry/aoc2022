use regex::Regex;

#[derive(Debug, Clone)]
pub enum Operation {
    Ls,
    Cd(String),
    ReturnDir(String),
    ReturnSize(u64),
}

pub fn import_data(data: &str) -> Vec<Operation> {
    data.lines().map(|line| parse_command(line)).collect()
}

pub fn parse_command(line: &str) -> Operation {
    let re = Regex::new(r"^\$?\s?(ls|cd|dir)\s?(.*)").unwrap();

    match re.captures(&line) {
        Some(caps) => {
            let command = caps
                .get(1)
                .and_then(|x| Some(x.as_str().to_owned()))
                .unwrap();
            match command.as_str() {
                "ls" => return Operation::Ls,
                "cd" => return Operation::Cd(caps.get(2).unwrap().as_str().to_owned()),
                "dir" => return Operation::ReturnDir(caps.get(2).unwrap().as_str().to_owned()),
                _ => unreachable!(),
            }
        }
        None => {
            // parse file:
            let re2 = Regex::new(r"^(\d+)\s(.+)").unwrap();

            let caps2 = re2.captures(&line).unwrap();

            Operation::ReturnSize(caps2.get(1).unwrap().as_str().parse().unwrap())
        }
    }
}

pub const TEST_DATA: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA);

        input_data
            .into_iter()
            .for_each(|operation| println!("{:?}", operation));
    }
}
