extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{iterators::Pair, Parser};
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CliParser;

struct File<'a> {
    name: &'a str,
    size: usize,
}

#[derive(Default)]
struct Directory<'a> {
    files: Vec<File<'a>>,
    dirs: Vec<&'a str>,
}

fn get_size(dir: &Directory<'_>) -> usize {
    dir.files.iter().map(|file| file.size).sum()
}

fn get_dirs_size(dir_db: &RefCell<BTreeMap<Vec<String>, Directory>>, root: &Vec<String>) -> usize {
    let local_db = dir_db.borrow();
    let directory = local_db.get(root).unwrap();
    let mut self_size = get_size(directory);
    for subdir in &directory.dirs {
        let mut next = root.clone();
        next.push(String::from(*subdir));
        self_size += get_dirs_size(dir_db, &next);
    }

    self_size
}
fn parse_input(input: Pair<Rule>) -> RefCell<BTreeMap<Vec<String>, Directory>> {
    let root_dir = Directory::default();
    let dir_db: RefCell<BTreeMap<Vec<String>, Directory>> = RefCell::new(BTreeMap::new());
    let mut current_pos = vec![String::from("/")];
    dir_db.borrow_mut().insert(current_pos.clone(), root_dir);
    for row in input.clone().into_inner() {
        if row.as_rule() == Rule::row {
            for record in row.into_inner() {
                match record.as_rule() {
                    Rule::command => {
                        match record.as_str() {
                            "cd /" => {
                                current_pos = vec![String::from("/")];
                                continue;
                            }
                            "cd .." => {
                                if current_pos.len() > 1 {
                                    current_pos.pop();
                                }
                                continue;
                            }
                            "ls" => {
                                continue;
                            }
                            _ => (),
                        };
                        for cmd in record.into_inner() {
                            match cmd.as_str() {
                                "cd" => {
                                    continue;
                                }
                                dirname => {
                                    let mut local_db = dir_db.borrow_mut();
                                    let newdir = Directory {
                                        files: Vec::new(),
                                        dirs: Vec::new(),
                                    };
                                    current_pos.push(String::from(dirname));
                                    local_db.insert(current_pos.clone(), newdir);
                                }
                            }
                        }
                    }
                    Rule::dir => {
                        let mut local_db = dir_db.borrow_mut();
                        let dir = local_db.get_mut(&current_pos).unwrap();
                        let mut name = "";
                        for pair in record.into_inner() {
                            name = pair.as_str();
                        }
                        dir.dirs.push(name);
                    }
                    Rule::file => {
                        let mut local_db = dir_db.borrow_mut();
                        let dir = local_db.get_mut(&current_pos).unwrap();
                        let mut file = File { name: "", size: 0 };
                        for pair in record.into_inner() {
                            match pair.as_rule() {
                                Rule::filename => {
                                    file.name = pair.as_str();
                                }
                                Rule::size => {
                                    file.size = pair.as_str().parse().unwrap();
                                }
                                _ => (),
                            }
                        }
                        dir.files.push(file);
                    }
                    _ => (),
                }
            }
        }
    }
    dir_db
}

fn main() {
    let unparsed_file = std::fs::read_to_string("../input.txt").expect("cannot read file");
    let file = CliParser::parse(Rule::all, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let dir_db = parse_input(file);
    let mut result_table: Vec<(&Vec<String>, usize)> = Vec::new();
    let db = dir_db.borrow();
    for (dirname, _dir) in db.iter() {
        result_table.push((dirname, get_dirs_size(&dir_db, &dirname.clone())));
    }

    let sum_of_less_than_100k: usize = result_table
        .iter()
        .filter(|(_, v)| v < &100_000usize)
        .map(|(_, v)| v)
        .sum();
    println!("Sum of dirs with less than 100k: {}", sum_of_less_than_100k);

    let total_diskspace = 70_000_000usize;
    let need = 30_000_000usize;
    let current_used = get_dirs_size(&dir_db, &vec![String::from("/")]);
    let current_unused = total_diskspace - current_used;
    let need_to_delete = need - current_unused;
    let mut filtered: Vec<(&Vec<String>, usize)> = result_table
        .into_iter()
        .filter(|(_, v)| v >= &need_to_delete)
        .collect();
    filtered.sort_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap());

    println!(
        "The directory to delete to free up {} is {}, which is of size {}",
        need_to_delete,
        filtered[0].0.last().unwrap(),
        filtered[0].1
    );
}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn parse_sample_input() {
        let input = "$ cd /
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
7214296 k";

        let sample_parsed = CliParser::parse(Rule::all, input)
            .expect("error parsing")
            .next()
            .unwrap();

        let dir_db = parse_input(sample_parsed);
        let mut result_table: Vec<(&str, usize)> = Vec::new();
        let db = dir_db.borrow();
        for (dirname, _dir) in db.iter() {
            result_table.push(("", get_dirs_size(&dir_db, &dirname.clone())));
        }

        let sum_of_less_than_100k: usize = result_table
            .iter()
            .filter(|(_, v)| v < &100_000usize)
            .map(|(_, v)| v)
            .sum();
        assert_eq!(95437, sum_of_less_than_100k);
        println!("Sum of dirs with less than 100k: {}", sum_of_less_than_100k);
    }
}
