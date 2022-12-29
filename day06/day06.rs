//!
//! [Day 6: Tuning Trouble](https://adventofcode.com/2022/day/6)
//!

use std::collections::HashSet;

struct Puzzle {
    signal: Vec<char>,
}

impl Puzzle {
    fn new() -> Self {
        Self { signal: Vec::new() }
    }

    fn configure(&mut self, path: &str) {
        let mut data = std::fs::read_to_string(path).unwrap();
        data.pop();
        self.signal = data.chars().collect();
    }

    fn find_marker(&self, length: usize) -> usize {
        let mut result = length;
        for slice in self.signal.windows(length) {
            // Transform the slice into a set to merge duplicated characters. If the size
            // stays the same, there is not duplicate entry
            let characters = slice.iter().copied().collect::<HashSet<_>>();
            if characters.len() == length {
                break;
            }
            result += 1;
        }
        result
    }

    fn part1(&self) -> usize {
        self.find_marker(4)
    }

    fn part2(&self) -> usize {
        self.find_marker(14)
    }
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test01.txt");
    assert_eq!(puzzle.part1(), 7);
    assert_eq!(puzzle.part2(), 19);
}

/// Test from puzzle input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), 5);
    assert_eq!(puzzle.part2(), 23);
}

/// Test from puzzle input
#[test]
fn test03() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test03.txt");
    assert_eq!(puzzle.part1(), 6);
    assert_eq!(puzzle.part2(), 23);
}

/// Test from puzzle input
#[test]
fn test04() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test04.txt");
    assert_eq!(puzzle.part1(), 10);
    assert_eq!(puzzle.part2(), 29);
}

/// Test from puzzle input
#[test]
fn test05() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test05.txt");
    assert_eq!(puzzle.part1(), 11);
    assert_eq!(puzzle.part2(), 26);
}

/// Test from user input
#[test]
fn test06() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test06.txt");
    assert_eq!(puzzle.part1(), 1356);
    assert_eq!(puzzle.part2(), 2564);
}

fn main() {
    let mut puzzle = Puzzle::new();
    let input = std::env::args().nth(1).expect("No input file");
    puzzle.configure(&input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
