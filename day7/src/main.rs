use regex::Regex;

#[macro_use]
extern crate derive_new;

fn import_data(data: &str) -> Vec<String> {
    data.lines().map(|x| x.into()).collect()
}

#[derive(new, Clone, Debug)]
struct Dir {
    pub name: String,
    pub filesize: u64,
    pub upper_directory: Option<String>,
    pub dirs: Vec<Dir>,
}

enum Operation {
    Ls,
    Cd(String),
    ReturnDir(String),
    ReturnSize(u64),
}

#[derive(Clone, Debug)]
struct Fs {
    pub dirs: Vec<Dir>,
    pub cursor: String,
}

fn parse_command(line: &str) -> Operation {
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

impl Default for Fs {
    fn default() -> Self {
        let path = "/".to_string();
        Self {
            dirs: vec![Dir::new(path.clone(), 0, None, vec![])],
            cursor: path,
        }
    }
}

impl Fs {
    fn get_current_directory(&mut self) -> &mut Dir {
        self.dirs
            .iter_mut()
            .find(|d| d.name == self.cursor)
            .unwrap()
    } // this is wrong! Just use a hashmap

    fn run_commands(&mut self, data: Vec<String>) {
        for line in data {
            match parse_command(&line) {
                Operation::Ls => (),
                Operation::ReturnDir(dir_name) => {
                    let new_directory = Dir::new(dir_name, 0, Some(self.cursor.clone()), vec![]);
                    let current_directory = self.get_current_directory();
                    current_directory.dirs.push(new_directory);
                }
                Operation::ReturnSize(size) => {
                    let current_directory = self.get_current_directory();
                    current_directory.filesize += size;
                }
                Operation::Cd(dir_name) => {
                    match dir_name.as_str() {
                        ".." => {
                            let current_directory = self.get_current_directory();
                            self.cursor = current_directory.upper_directory.clone().unwrap();
                        }
                        "/" => {
                            self.cursor = "/".to_string();
                        }
                        x_directory => {
                            println!("x dir: {:?}", &self.dirs);
                            let current_directory = self.get_current_directory();
                            let cd_directory = current_directory
                                .dirs
                                .iter_mut()
                                .find(|d| d.name == x_directory)
                                .unwrap();
                            self.cursor = cd_directory.name.clone();
                        }
                    };
                }
            }
        }
    }

    fn calculate_result(&self) -> u64 {
        calculate_dir_sizes(&self.dirs, 0)
    }
}

fn calculate_dir_sizes(sub_dirs: &Vec<Dir>, size: u64) -> u64 {
    for dir in sub_dirs {
        let recursive_dir_sizes = calculate_dir_sizes(&dir.dirs, dir.filesize + size);
        if recursive_dir_sizes <= 100000 {
            println!("recursive: {}", recursive_dir_sizes);
            return recursive_dir_sizes;
        } else {
            return 0;
        }
    }

    if size <= 100000 {
        println!("size: {}", size);
        size
    } else {
        0
    }
}

fn answer_part1(data: Vec<String>) -> u64 {
    let mut directory = Fs::default();

    println!("{:?}", &directory.dirs);

    directory.run_commands(data);

    println!("{:?}", &directory.dirs);

    directory.calculate_result()
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
