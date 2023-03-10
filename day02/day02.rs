//!
//! [Day 2: Rock Paper Scissors](https://adventofcode.com/2022/day/2)
//!

const VALUE_ROCK: u32 = 1;
const VALUE_PAPER: u32 = 2;
const VALUE_SCISSORS: u32 = 3;

const SHOULD_LOSE: u32 = 1;
const SHOULD_DRAW: u32 = 2;

const ROUND_OUTCOME_DRAW: u32 = 3;
const ROUND_OUTCOME_WIN: u32 = 6;

struct Puzzle {
    guide: Vec<(u32, u32)>,
}

impl Puzzle {
    fn new() -> Self {
        Self { guide: Vec::new() }
    }

    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let mut lines = data.split('\n').collect::<Vec<_>>();
        lines.pop();

        for strategy in lines {
            let shapes = strategy.split(' ').collect::<Vec<_>>();
            let opponent = match shapes[0].parse::<char>().unwrap() {
                'A' => VALUE_ROCK,
                'B' => VALUE_PAPER,
                'C' => VALUE_SCISSORS,
                _ => panic!("bad input"),
            };
            let you = match shapes[1].parse::<char>().unwrap() {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => panic!("bad input"),
            };
            self.guide.push((opponent, you));
        }
    }

    fn part1(&self) -> u32 {
        let mut result = 0;
        for strategy in &self.guide {
            let (opponent, you) = *strategy;

            // Always win what has been played
            result += you;

            // Take advantage of shapes sorting to check win conditions
            if opponent + 1 == you || you + 2 == opponent {
                result += ROUND_OUTCOME_WIN;
            } else if opponent == you {
                result += ROUND_OUTCOME_DRAW;
            }
        }
        result
    }

    fn part2(&self) -> u32 {
        let mut result = 0;
        for strategy in &self.guide {
            let (opponent, you) = *strategy;

            if you == SHOULD_LOSE {
                if opponent == VALUE_ROCK {
                    result += VALUE_SCISSORS;
                } else {
                    // Take advantage of shapes sorting to know what to play
                    result += opponent - 1;
                }
            } else if you == SHOULD_DRAW {
                result += ROUND_OUTCOME_DRAW + opponent;
            } else {
                result += ROUND_OUTCOME_WIN;
                if opponent == VALUE_SCISSORS {
                    result += VALUE_ROCK;
                } else {
                    // Take advantage of shapes sorting to know what to play
                    result += opponent + 1;
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
    assert_eq!(puzzle.part1(), 15);
    assert_eq!(puzzle.part2(), 12);
}

/// Test from user input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), 15337);
    assert_eq!(puzzle.part2(), 11696);
}

/// Test from user input
#[test]
fn test03() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test03.txt");
    assert_eq!(puzzle.part1(), 12156);
    assert_eq!(puzzle.part2(), 10835);
}

fn main() {
    let mut puzzle = Puzzle::new();
    let input = std::env::args().nth(1).expect("No input file");
    puzzle.configure(&input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
