//!
//! [Day 12: Hill Climbing Algorithm](https://adventofcode.com/2022/day/12)
//!

use std::collections::{HashSet, VecDeque};

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
}

struct Path {
    position: (usize, usize),
    cost: usize,
}

struct Search {
    direction: Direction,
    paths: VecDeque<Path>,
    visited: HashSet<(usize, usize)>,
}

struct Puzzle {
    heightmap: Vec<Vec<u32>>,
    cols: usize,
    rows: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Puzzle {
    fn new() -> Self {
        Self {
            heightmap: Vec::new(),
            cols: 0,
            rows: 0,
            start: (0, 0),
            end: (0, 0),
        }
    }

    fn configure(&mut self, path: &str) {
        let mut data = std::fs::read_to_string(path).unwrap();
        data.pop();

        self.heightmap = data
            .split('\n')
            .map(|l| l.chars().map(|c| c as u32).collect::<Vec<_>>())
            .collect();

        self.rows = self.heightmap.len();
        self.cols = self.heightmap[0].len();

        for (x, line) in self.heightmap.iter_mut().enumerate() {
            for (y, c) in line.iter_mut().enumerate() {
                if *c == 'S' as u32 {
                    self.start = (x, y);
                    *c = 'a' as u32;
                } else if *c == 'E' as u32 {
                    self.end = (x, y);
                    *c = 'z' as u32;
                }
            }
        }
    }

    fn push_path(&self, search: &mut Search, from: &Path, to: (usize, usize)) -> bool {
        // Check elevation
        if search.direction == Direction::Up
            && self.heightmap[from.position.0][from.position.1] + 1 < self.heightmap[to.0][to.1]
            || search.direction == Direction::Down
                && self.heightmap[to.0][to.1] + 1 < self.heightmap[from.position.0][from.position.1]
        {
            return false;
        }

        // Push new path if it has not been visited yet
        if search.visited.insert(to) {
            search.paths.push_back(Path {
                position: to,
                cost: from.cost + 1,
            });
        }

        true
    }

    fn fewest_steps_to_end(&self, from: (usize, usize), to: &[(usize, usize)]) -> usize {
        let mut search = Search {
            direction: Direction::Up,
            paths: VecDeque::new(),
            visited: HashSet::new(),
        };
        search.visited.insert(from);
        search.paths.push_back(Path {
            position: from,
            cost: 0,
        });
        search.direction = if from == self.start {
            Direction::Up
        } else {
            Direction::Down
        };

        // We suppose the destination is always on the same level
        let to_level = self.heightmap[to[0].0][to[0].1];

        let mut steps = usize::MAX;
        while !search.paths.is_empty() && steps == usize::MAX {
            let path = search.paths.pop_front().unwrap();

            let mut new_pos = Vec::new();
            if path.position.0 + 1 < self.rows {
                new_pos.push((path.position.0 + 1, path.position.1));
            }
            if path.position.0 > 0 {
                new_pos.push((path.position.0 - 1, path.position.1));
            }
            if path.position.1 + 1 < self.cols {
                new_pos.push((path.position.0, path.position.1 + 1));
            }
            if path.position.1 > 0 {
                new_pos.push((path.position.0, path.position.1 - 1));
            }

            for pos in &new_pos {
                if self.push_path(&mut search, &path, *pos)
                    && self.heightmap[pos.0][pos.1] == to_level
                    && to.contains(pos)
                {
                    steps = path.cost + 1;
                }
            }
        }

        steps
    }

    fn part1(&self) -> usize {
        let to = vec![self.end];
        self.fewest_steps_to_end(self.start, &to)
    }

    fn part2(&self) -> usize {
        let mut to = Vec::new();
        for (i, row) in self.heightmap.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if *val == 'a' as u32 {
                    to.push((i, j));
                }
            }
        }

        self.fewest_steps_to_end(self.end, &to)
    }
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test01.txt");
    assert_eq!(puzzle.part1(), 31);
    assert_eq!(puzzle.part2(), 29);
}

/// Test from user input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), 497);
    assert_eq!(puzzle.part2(), 492);
}

fn main() {
    let mut puzzle = Puzzle::new();
    let input = std::env::args().nth(1).expect("No input file");
    puzzle.configure(&input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
