//!
//! [Day 1: Calorie Counting](https://adventofcode.com/2022/day/1)
//!

struct Puzzle {
    calories: Vec<usize>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            calories: Vec::new(),
        }
    }

    fn configure(&mut self, path: &str) {
        let mut data = std::fs::read_to_string(path).unwrap();
        data.pop();
        self.calories = data
            .split("\n\n")
            .map(|x| {
                x.split('\n')
                    .map(|y| y.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .collect::<Vec<_>>();

        // Reverse sort to have to most significant values first
        self.calories.sort_by(|a, b| b.cmp(a));
    }

    fn part1(&self) -> usize {
        self.calories[0]
    }

    fn part2(&self) -> usize {
        self.calories[0..3].iter().sum::<usize>()
    }
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test01.txt");
    assert_eq!(puzzle.part1(), 24000);
    assert_eq!(puzzle.part2(), 45000);
}

/// Test from user input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), 69310);
    assert_eq!(puzzle.part2(), 206104);
}

/// Test from user input
#[test]
fn test03() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test03.txt");
    assert_eq!(puzzle.part1(), 72070);
    assert_eq!(puzzle.part2(), 211805);
}

fn main() {
    let mut puzzle = Puzzle::new();
    let input = std::env::args().nth(1).expect("No input file");
    puzzle.configure(&input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
