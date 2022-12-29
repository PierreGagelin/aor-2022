//!
//! [Day 7: No Space Left On Device](https://adventofcode.com/2022/day/7)
//!

use std::collections::HashMap;

const FILESYSTEM_SIZE: usize = 70_000_000;
const UPDATE_SIZE: usize = 30_000_000;

struct Puzzle {
    directories_size: Vec<usize>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            directories_size: Vec::new(),
        }
    }

    fn configure(&mut self, path: &str) {
        let mut data = std::fs::read_to_string(path).unwrap();
        data.pop();

        let mut directories = HashMap::from([(String::from("/"), 0)]);

        // Get the size of directories (excluding subdirectories)
        let mut current_dir = Vec::new();
        for line in data.split('\n') {
            if line == "$ cd /" {
                current_dir = vec![String::from("/")];
            } else if line == "$ cd .." {
                current_dir.pop();
                current_dir.pop();
            } else if let Some((_, dir)) = line.split_once("$ cd ") {
                current_dir.push(dir.to_string());
                current_dir.push(String::from("/"));
            } else if line == "$ ls" {
                // Do nothing special, next entries will be parsed according to the current directory
            } else if let Some((_, name)) = line.split_once("dir ") {
                let mut dirname = current_dir.concat();
                dirname.push_str(name);
                dirname.push('/');
                if directories.get(&dirname) == None {
                    directories.insert(dirname, 0);
                }
            } else {
                // This is a file entry description
                let (size, _) = line.split_once(' ').unwrap();
                let fullpath = current_dir.concat();
                let dir_size = directories.get_mut(&fullpath).unwrap();
                *dir_size += size.parse::<usize>().unwrap();
            }
        }

        // Compute the size of directories (including subdirectories)
        for path in directories.keys() {
            let dir_size = directories
                .iter()
                .filter_map(|(k, v)| if k.starts_with(path) { Some(*v) } else { None })
                .sum();
            self.directories_size.push(dir_size);
        }
    }

    fn part1(&self) -> usize {
        self.directories_size
            .iter()
            .fold(0, |a, i| if *i <= 100_000 { a + i } else { a })
    }

    fn part2(&self) -> usize {
        let used_space = self.directories_size.iter().max().unwrap();
        let required_space = UPDATE_SIZE - (FILESYSTEM_SIZE - used_space);

        self.directories_size.iter().fold(FILESYSTEM_SIZE, |a, i| {
            if *i > required_space && *i < a {
                *i
            } else {
                a
            }
        })
    }
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test01.txt");
    assert_eq!(puzzle.part1(), 95437);
    assert_eq!(puzzle.part2(), 24933642);
}

/// Test from user input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), 1297159);
    assert_eq!(puzzle.part2(), 3866390);
}

fn main() {
    let mut puzzle = Puzzle::new();
    let input = std::env::args().nth(1).expect("No input file");
    puzzle.configure(&input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
