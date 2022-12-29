//!
//! [Day 8: Treetop Tree House](https://adventofcode.com/2022/day/8)
//!

struct Puzzle {
    rows: Vec<Vec<u32>>,
    columns: Vec<Vec<u32>>,
}

fn is_visible(index: usize, line: &[u32]) -> bool {
    // Check `[0, index)`
    if let Some(max) = line.get(..index).unwrap().iter().max() {
        if *max < line[index] {
            return true;
        }
    } else {
        return true;
    }

    // Check `(index, end]`
    if let Some(max) = line.get(index + 1..).unwrap().iter().max() {
        if *max < line[index] {
            return true;
        }
    } else {
        return true;
    }

    false
}

fn scenic(index: usize, line: &[u32]) -> u32 {
    let mut begin_score = 0;
    for i in 1..=index {
        begin_score += 1;
        if line[index - i] >= line[index] {
            break;
        }
    }

    let mut end_score = 0;
    for i in index + 1..line.len() {
        end_score += 1;
        if line[i] >= line[index] {
            break;
        }
    }

    begin_score * end_score
}

impl Puzzle {
    fn new() -> Self {
        Self {
            rows: Vec::new(),
            columns: Vec::new(),
        }
    }

    fn configure(&mut self, path: &str) {
        let mut data = std::fs::read_to_string(path).unwrap();
        data.pop();

        // Initialize columns
        for _ in 0..data.find('\n').unwrap() {
            self.columns.push(Vec::new());
        }

        // Do two representations of the forest, on by rows and the other by columns
        for line in data.split('\n') {
            self.rows.push(Vec::new());
            for (index, c) in line.chars().enumerate() {
                let val = c.to_digit(10).unwrap();
                self.rows.last_mut().unwrap().push(val);
                self.columns[index].push(val);
            }
        }
    }

    fn part1(&self) -> usize {
        let mut visible_tree_count = 0;
        for r in 0..self.rows.len() {
            for c in 0..self.rows[r].len() {
                if is_visible(c, &self.rows[r]) || is_visible(r, &self.columns[c]) {
                    visible_tree_count += 1;
                }
            }
        }
        visible_tree_count
    }

    fn part2(&self) -> u32 {
        let mut scenic_max = 0;

        // Ignore borders during iteration, scenic score is null
        for r in 1..self.rows.len() - 1 {
            for c in 1..self.rows[r].len() - 1 {
                scenic_max = scenic_max.max(scenic(c, &self.rows[r]) * scenic(r, &self.columns[c]));
            }
        }

        scenic_max
    }
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test01.txt");
    assert_eq!(puzzle.part1(), 21);
    assert_eq!(puzzle.part2(), 8);
}

/// Test from user input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), 1849);
    assert_eq!(puzzle.part2(), 201600);
}

fn main() {
    let mut puzzle = Puzzle::new();
    let input = std::env::args().nth(1).expect("No input file");
    puzzle.configure(&input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
