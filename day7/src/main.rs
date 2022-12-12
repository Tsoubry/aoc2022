pub mod data;

#[macro_use]
extern crate derive_new;

use std::collections::HashMap;

use regex::Regex;

use crate::data::*;

#[derive(new, Clone, Debug)]
pub struct DirInfo {
    pub upper_directory: Option<String>,
    pub filesize: u64,
    pub level: usize,
}

fn get_directory_sizes(data: Vec<Operation>) -> HashMap<String, DirInfo> {
    let mut fs: HashMap<String, DirInfo> = HashMap::new();
    fs.insert("/".to_string(), DirInfo::new(None, 0, 0));

    let mut current_level = 0;
    let mut path: String = "/".to_string();

    for command in data {
        match command {
            Operation::Ls => (),
            Operation::ReturnDir(dir_name) => {
                let new_directory = DirInfo::new(Some(path.clone()), 0, current_level + 1);
                let new_path = format!("{}/{}", path, dir_name);
                fs.entry(new_path)
                    .and_modify(|_| ())
                    .or_insert(new_directory);
            }
            Operation::ReturnSize(size) => {
                fs.entry(path.clone())
                    .and_modify(|meta| meta.filesize += size);
            }
            Operation::Cd(dir_name) => {
                match dir_name.as_str() {
                    ".." => {
                        let re_upper = Regex::new(r"(.+)/[a-zA-Z0-9]+$").unwrap();
                        let caps = re_upper.captures(&path).unwrap().get(1).unwrap().as_str();
                        current_level -= 1;
                        path = format!("{}", caps);
                    }
                    "/" => {
                        current_level = 0;
                        path = "/".to_string();
                    }
                    x_directory => {
                        current_level += 1;
                        path = format!("{}/{}", path, x_directory);
                    }
                };
            }
        }
    }

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

    fs
}

fn answer_part1(data: Vec<Operation>) -> u64 {
    let fs = get_directory_sizes(data);

    fs.into_iter()
        .map(|(_, v)| v.filesize)
        .filter(|filesize| filesize <= &100_000)
        .sum()
}

fn answer_part2(data: Vec<Operation>) -> u64 {
    let total_size: u64 = 70_000_000;
    let space_needed: u64 = 30_000_000;

    let fs = get_directory_sizes(data);

    let space_used = fs.get(&"/".to_string()).unwrap().filesize;

    let space_to_free = space_needed - (total_size - space_used);

    let mut directory_candidates: Vec<_> = fs
        .into_iter()
        .map(|(_, v)| v.filesize)
        .filter(|filesize| filesize > &space_to_free)
        .collect();

    directory_candidates.sort();

    *directory_candidates.first().unwrap()
}

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone())); // not 1588580
    println!("Answer of part 2 is: {}", answer_part2(input_data));
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

    #[test]
    fn test_answer2() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(24933642, answer_part2(input_data));
    }
}
