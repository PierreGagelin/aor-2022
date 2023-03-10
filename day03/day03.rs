//!
//! [Day 3: Rucksack Reorganization](https://adventofcode.com/2022/day/3)
//!

struct Puzzle {
    rucksacks: Vec<String>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            rucksacks: Vec::new(),
        }
    }

    fn char_to_priority(c: char) -> u32 {
        match c {
            'A'..='Z' => 27 + u32::from(c) - u32::from('A'),
            _ => 1 + u32::from(c) - u32::from('a'),
        }
    }

    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let mut lines = data.split('\n').collect::<Vec<_>>();
        lines.pop();
        self.rucksacks = lines.iter().map(std::string::ToString::to_string).collect();
    }

    fn part1(&self) -> u32 {
        let mut result = 0;
        for rucksack in &self.rucksacks {
            let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);
            for c in first_compartment.chars() {
                if second_compartment.contains(c) {
                    result += Puzzle::char_to_priority(c);
                    break;
                }
            }
        }
        result
    }

    fn part2(&self) -> u32 {
        let mut result = 0;

        // Iterate over rucksacks by triples
        for slice in self.rucksacks.chunks(3) {
            let first = &slice[0];
            let second = &slice[1];
            let third = &slice[2];
            for c in first.chars() {
                // Look for the character in the two others
                if second.contains(c) && third.contains(c) {
                    result += Puzzle::char_to_priority(c);
                    break;
                }
            }
        }
        result
    }
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test01.txt");
    assert_eq!(puzzle.part1(), 157);
    assert_eq!(puzzle.part2(), 70);
}

/// Test from user input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), 7831);
    assert_eq!(puzzle.part2(), 2683);
}

fn main() {
    let mut puzzle = Puzzle::new();
    let input = std::env::args().nth(1).expect("No input file");
    puzzle.configure(&input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
