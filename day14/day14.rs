//!
//! [Day 14: Regolith Reservoir](https://adventofcode.com/2022/day/14)
//!

fn move_sand(cave: &mut Vec<Vec<char>>, sand_position: &mut (usize, usize)) -> bool {
    let mut blocked = false;

    if cave[sand_position.1 + 1][sand_position.0] == '.' {
        sand_position.1 += 1;
    } else if cave[sand_position.1 + 1][sand_position.0 - 1] == '.' {
        sand_position.1 += 1;
        sand_position.0 -= 1;
    } else if cave[sand_position.1 + 1][sand_position.0 + 1] == '.' {
        sand_position.1 += 1;
        sand_position.0 += 1;
    } else {
        // Sand cannot move further
        blocked = true;
        cave[sand_position.1][sand_position.0] = 'o';
        *sand_position = (500, 0);
    }

    blocked
}

struct Puzzle {
    cave: Vec<Vec<char>>,
    floor: usize,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            cave: Vec::new(),
            floor: 2,
        }
    }

    fn configure(&mut self, path: &str) {
        let mut data = std::fs::read_to_string(path).unwrap();
        data.pop();

        // Not interested in the cave size, never more than 1000
        self.cave.resize(1000, Vec::new());
        for row in &mut self.cave {
            row.resize(1000, '.');
        }

        // Place rocks
        for path in data.split('\n') {
            let mut pos: Option<(usize, usize)> = None;
            for line in path.split(" -> ") {
                let (x, y) = line.split_once(',').unwrap();
                let (to_x, to_y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
                if let Some((from_x, from_y)) = pos {
                    for i in from_y.min(to_y)..=from_y.max(to_y) {
                        self.cave[i][from_x] = '#';
                    }
                    for i in from_x.min(to_x)..=from_x.max(to_x) {
                        self.cave[from_y][i] = '#';
                    }
                }
                pos = Some((to_x, to_y));
                self.floor = self.floor.max(to_y + 2);
            }
        }
    }

    fn part1(&self) -> u32 {
        let mut cave = self.cave.clone();
        let mut sand_count = 0;
        let mut sand_position = (500, 0);
        while sand_position.1 < 999 {
            if move_sand(&mut cave, &mut sand_position) {
                sand_count += 1;
            }
        }
        sand_count
    }

    fn part2(&self) -> u32 {
        let mut cave = self.cave.clone();

        for x in 0..1000 {
            cave[self.floor][x] = '#';
        }

        let mut sand_count = 0;
        let mut sand_position = (500, 0);
        while cave[0][500] != 'o' {
            if move_sand(&mut cave, &mut sand_position) {
                sand_count += 1;
            }
        }
        sand_count
    }
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test01.txt");
    assert_eq!(puzzle.part1(), 24);
    assert_eq!(puzzle.part2(), 93);
}

/// Test from user input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), 961);
    assert_eq!(puzzle.part2(), 26375);
}

fn main() {
    let mut puzzle = Puzzle::new();
    let input = std::env::args().nth(1).expect("No input file");
    puzzle.configure(&input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
