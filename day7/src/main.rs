use regex::Regex;

#[macro_use]
extern crate derive_new;

#[derive(new)]
struct Dir {
    pub name: String,
    pub filesize: u64,
    pub dirs: Vec<Dir>,
}

enum Operation {
    Ls,
    Cd(String),
    ReturnDir(String),
    ReturnSize(u64),
}

fn import_data(data: &str) -> Vec<String> {
    data.lines().map(|x| x.into()).collect()
}

fn parse_command(line: &str) -> Operation {
    let re = Regex::new(r"^\$?\s?(ls|cd|dir)\s?(.+)").unwrap();

    if let Some(caps) = re.captures(&line) {
        let command: Option<String> = caps.get(1).and_then(|x| Some(x.as_str().to_owned()));

        if let Some(cmd) = command {
            match cmd.as_str() {
                "ls" => return Operation::Ls,
                "cd" => return Operation::Cd(caps.get(2).unwrap().as_str().to_owned()),
                "dir" => return Operation::ReturnDir(caps.get(2).unwrap().as_str().to_owned()),
                _ => unreachable!(),
            }
        };
    }

    // parse file:
    let re2 = Regex::new(r"^(\d+)\s(.+)").unwrap();

    let caps2 = re2.captures(&line).unwrap();

    return Operation::ReturnSize(caps2.get(1).unwrap().as_str().parse().unwrap())


}

fn parse_size(line: &str) -> Option<u64> {
    let re = Regex::new(r"^(\d+)\s(.+)").unwrap();

    if let Some(caps) = re.captures(&line) {
        return caps.get(1).and_then(|x| x.as_str().parse().ok());
    }

    None
}

fn answer_part1(data: Vec<String>) -> u64 {
    let mut directory = Dir {
        name: "/".to_string(),
        filesize: 0,
        dirs: vec![],
    };

    let mut current_dir: &mut Dir = &mut directory;

    let mut last_operation = Operation::Cd("/".to_string());

    for line in data {
        if true {
            // cd
            let cd = current_dir
                .dirs
                .iter_mut()
                .find(|d| d.name == "abc".to_string())
                .unwrap();
            current_dir = cd;
        }

        // if let Some(dir) = parse_command(&line) {
        //     directory.

        // };

        if let Some(size) = parse_size(&line) {
            current_dir.filesize += size;
        };
    }

    if current_dir_size <= 100000 {
        dir_sizes_small.push(current_dir_size);
    }
    current_dir_size = 0;

    dir_sizes_small.iter().sum()
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

    const TEST_DATA: &str = r#"$ cd /
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

    #[test]
    fn test_answer1() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(95437, answer_part1(input_data));
    }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA);
    //     assert_eq!(, answer_part2(input_data));
    // }

    #[test]
    fn playground() {}
}
