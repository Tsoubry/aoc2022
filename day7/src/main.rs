pub mod data;

#[macro_use]
extern crate derive_new;

use std::collections::HashMap;

use crate::data::*;

#[derive(new, Clone, Debug)]
pub struct DirInfo {
    pub upper_directory: Option<String>,
    pub filesize: u64,
    pub level: usize,
}

fn answer_part1(data: Vec<Operation>) -> u64 {
    let mut fs: HashMap<String, DirInfo> = HashMap::new();
    fs.insert("/".to_string(), DirInfo::new(None, 0, 0));

    let mut cwd: Option<String> = None;
    let mut current_level = 0;

    for command in data {
        let refcwd = &mut cwd;
        match command {
            Operation::Ls => (),
            Operation::ReturnDir(dir_name) => {
                let new_directory = DirInfo::new(refcwd.clone(), 0, current_level + 1);
                fs.entry(dir_name)
                    .and_modify(|_| ())
                    .or_insert(new_directory);
            }
            Operation::ReturnSize(size) => {
                fs.entry(refcwd.clone().expect("invalid key in returnsize"))
                    .and_modify(|meta| meta.filesize += size);
            }
            Operation::Cd(dir_name) => {
                match dir_name.as_str() {
                    ".." => {
                        let upper_dir = fs
                            .get(&refcwd.clone().unwrap())
                            .expect("key doesn't exist for .. operation")
                            .upper_directory
                            .clone();
                        cwd = upper_dir;
                        current_level -= 1;
                    }
                    "/" => {
                        cwd = Some("/".to_string());
                        current_level = 0;
                    }
                    x_directory => {
                        assert!(fs.contains_key(x_directory));
                        cwd = Some(x_directory.to_string());
                        current_level += 1;
                    }
                };
            }
        }
    }

    // println!("Filesystem: {:?}", &fs);

    let mut directories: Vec<_> = fs.clone().into_iter().collect();
    directories.sort_by_key(|(_, meta)| meta.level);
    directories.reverse();

    for (dirname, meta) in directories.into_iter() {
        let mut filesize: u64;

        {
            filesize = fs.get(&dirname).clone().unwrap().filesize;
        }

        if let Some(upper_directory) = meta.upper_directory {
            fs.entry(upper_directory)
                .and_modify(|dir| dir.filesize += filesize);
        };
    }

    println!("Filesystem: {:?}", &fs);

    fs.into_iter()
        .map(|(_, v)| v.filesize)
        .filter(|filesize| filesize <= &100_000)
        .sum()
}

// fn answer_part2(data: Vec<Parsed>) -> i64 {

// }

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone())); // not 1588580
    // println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::data::TEST_DATA;

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
