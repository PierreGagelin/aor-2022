//!
//! [Day 5: Supply Stacks](https://adventofcode.com/2022/day/5)
//!

struct Puzzle {
    crates_stacks: Vec<Vec<char>>,
    instructions: Vec<(usize, usize, usize)>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            crates_stacks: Vec::new(),
            instructions: Vec::new(),
        }
    }

    fn configure(&mut self, path: &str) {
        let mut data = std::fs::read_to_string(path).unwrap();
        data.pop();
        let (crates, instructions) = data.split_once("\n\n").unwrap();
        let mut crates = crates.split('\n').collect::<Vec<_>>();

        // The character before the last on the last line is the count of stacks
        let stack_count = crates.pop().unwrap().chars().nth_back(1).unwrap();
        let stack_count = stack_count.to_digit(10).unwrap() as usize;
        for _ in 0..stack_count {
            self.crates_stacks.push(Vec::new());
        }

        // Fill the crates stacks
        for crate_line in crates.iter().rev() {
            for stack_index in 0..stack_count {
                // Crate character is every 4 characters starting at the second
                let c = crate_line.chars().nth(1 + 4 * stack_index).unwrap();
                if c.is_ascii_alphabetic() {
                    self.crates_stacks[stack_index].push(c);
                }
            }
        }

        // Split instructions by whitespaces to get count of crates to move, source and destination stacks
        let instructions = instructions.split('\n').collect::<Vec<_>>();
        for instruction_line in instructions {
            let mut split = instruction_line.split_ascii_whitespace();
            let count = split.nth(1).unwrap().parse::<usize>().unwrap();
            let from = split.nth(1).unwrap().parse::<usize>().unwrap();
            let to = split.nth(1).unwrap().parse::<usize>().unwrap();
            self.instructions.push((count, from, to));
        }
    }

    fn part1(&self) -> String {
        let mut result = String::new();

        let mut crates_stacks = self.crates_stacks.clone();

        for &(count, from, to) in &self.instructions {
            for _ in 0..count {
                let c = crates_stacks[from - 1].pop().unwrap();
                crates_stacks[to - 1].push(c);
            }
        }

        for stack in &mut crates_stacks {
            if let Some(c) = stack.pop() {
                result.push(c);
            }
        }

        result
    }

    fn part2(&self) -> String {
        let mut result = String::new();

        let mut crates_stacks = self.crates_stacks.clone();

        for &(count, from, to) in &self.instructions {
            let stack_len = crates_stacks[from - 1].len();
            let mut moved = crates_stacks[from - 1]
                .get(stack_len - count..)
                .unwrap()
                .to_vec();
            crates_stacks[to - 1].append(&mut moved);
            crates_stacks[from - 1].truncate(stack_len - count);
        }

        for stack in &mut crates_stacks {
            if let Some(c) = stack.pop() {
                result.push(c);
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
    assert_eq!(puzzle.part1(), "CMZ");
    assert_eq!(puzzle.part2(), "MCD");
}

/// Test from user input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), "RFFFWBPNS");
    assert_eq!(puzzle.part2(), "CQQBBJFCS");
}

fn main() {
    let mut puzzle = Puzzle::new();
    let input = std::env::args().nth(1).expect("No input file");
    puzzle.configure(&input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
