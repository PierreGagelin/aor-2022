//!
//! [Day 9: Rope Bridge](https://adventofcode.com/2022/day/9)
//!

use std::collections::HashSet;

struct Puzzle {
    motions: Vec<char>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            motions: Vec::new(),
        }
    }

    fn configure(&mut self, path: &str) {
        let mut data = std::fs::read_to_string(path).unwrap();
        data.pop();

        for line in data.split('\n') {
            let (direction, count) = line.split_once(' ').unwrap();
            let (direction, count) = (
                direction.parse::<char>().unwrap(),
                count.parse::<usize>().unwrap(),
            );

            // Split a motion into several 1-step motions
            for _ in 0..count {
                self.motions.push(direction);
            }
        }
    }

    fn move_rope(&self, length: usize) -> usize {
        let mut knots_pos: Vec<(i32, i32)> = Vec::new();
        knots_pos.resize(length, (0, 0));

        let mut positions = HashSet::new();
        positions.insert((0, 0));

        for direction in &self.motions {
            let head_position = knots_pos.get_mut(0).unwrap();
            match direction {
                'U' => head_position.1 += 1,
                'D' => head_position.1 -= 1,
                'L' => head_position.0 -= 1,
                'R' => head_position.0 += 1,
                _ => panic!("bad input"),
            }

            // Iterate over pairs of knots. The first acting like an head and the other like a tail
            for index in 0..length - 1 {
                let head = knots_pos[index];
                let tail = knots_pos.get_mut(index + 1).unwrap();

                // Check if `head` is on the 5x5 square around `tail`
                if (head.1 - tail.1).abs() == 2 && (head.0 - tail.0).abs() == 2 {
                    tail.0 = (tail.0 + head.0) / 2;
                    tail.1 = (tail.1 + head.1) / 2;
                } else if (head.1 - tail.1).abs() == 2 {
                    tail.0 = head.0;
                    tail.1 = (tail.1 + head.1) / 2;
                } else if (head.0 - tail.0).abs() == 2 {
                    tail.0 = (tail.0 + head.0) / 2;
                    tail.1 = head.1;
                } else {
                    // This knot does not move, there is no need to check the others
                    break;
                }
            }
            positions.insert(*knots_pos.last().unwrap());
        }

        positions.len()
    }

    fn part1(&self) -> usize {
        self.move_rope(2)
    }

    fn part2(&self) -> usize {
        self.move_rope(10)
    }
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test01.txt");
    assert_eq!(puzzle.part1(), 13);
    assert_eq!(puzzle.part2(), 1);
}

/// Test from puzzle input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part2(), 36);
}

/// Test from user input
#[test]
fn test03() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test03.txt");
    assert_eq!(puzzle.part1(), 5883);
    assert_eq!(puzzle.part2(), 2367);
}

fn main() {
    let mut puzzle = Puzzle::new();
    let input = std::env::args().nth(1).expect("No input file");
    puzzle.configure(&input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
