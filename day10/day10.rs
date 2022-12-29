//!
//! [Day 10: Cathode-Ray Tube](https://adventofcode.com/2022/day/10)
//!

struct Puzzle {
    instructions: Vec<i32>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    fn configure(&mut self, path: &str) {
        let mut data = std::fs::read_to_string(path).unwrap();
        data.pop();

        // The idea applied here is to make each entry in the vector count as one cycle
        for line in data.split('\n') {
            if line == "noop" {
                self.instructions.push(0);
            } else {
                // Add an empty instruction to simulate the first cycle
                self.instructions.push(0);
                let (_, val) = line.split_once(' ').unwrap();
                self.instructions.push(val.parse::<i32>().unwrap());
            }
        }
    }

    fn part1(&self) -> i32 {
        let mut signal_strength = 0;
        let mut register_value: i32 = 1;

        for (index, value) in self.instructions.iter().enumerate() {
            let cycle = i32::try_from(index).unwrap() + 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                signal_strength += register_value * cycle;
            }
            register_value += value;
        }

        signal_strength
    }

    fn part2(&self) -> String {
        let mut sprite_position = 2;
        let mut row_offset = 0;
        let mut crt = String::new();

        for (index, value) in self.instructions.iter().enumerate() {
            let cycle = i32::try_from(index).unwrap() + 1 - row_offset;
            if (sprite_position - 1..=sprite_position + 1).contains(&cycle) {
                crt.push('#');
            } else {
                crt.push('.');
            }
            if cycle == 40 {
                crt.push('\n');
                row_offset += 40;
            }
            sprite_position += value;
        }

        crt
    }
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test01.txt");
    assert_eq!(puzzle.part1(), 13140);

    let mut expected = String::new();
    expected.push_str("##..##..##..##..##..##..##..##..##..##..\n");
    expected.push_str("###...###...###...###...###...###...###.\n");
    expected.push_str("####....####....####....####....####....\n");
    expected.push_str("#####.....#####.....#####.....#####.....\n");
    expected.push_str("######......######......######......####\n");
    expected.push_str("#######.......#######.......#######.....\n");
    assert_eq!(puzzle.part2(), expected);
}

/// Test from user input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), 16060);

    let mut expected = String::new();
    expected.push_str("###...##...##..####.#..#.#....#..#.####.\n");
    expected.push_str("#..#.#..#.#..#.#....#.#..#....#..#.#....\n");
    expected.push_str("###..#..#.#....###..##...#....####.###..\n");
    expected.push_str("#..#.####.#....#....#.#..#....#..#.#....\n");
    expected.push_str("#..#.#..#.#..#.#....#.#..#....#..#.#....\n");
    expected.push_str("###..#..#..##..####.#..#.####.#..#.#....\n");
    assert_eq!(puzzle.part2(), expected);
}

fn main() {
    let mut puzzle = Puzzle::new();
    let input = std::env::args().nth(1).expect("No input file");
    puzzle.configure(&input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
