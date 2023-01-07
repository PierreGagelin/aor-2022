//!
//! [Day 11: Monkey in the Middle](https://adventofcode.com/2022/day/11)
//!

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    multiply: bool,
    worry_operand: Option<u64>,
    divisibility: u64,
    true_to: usize,
    false_to: usize,
}

impl Monkey {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            multiply: false,
            worry_operand: None,
            divisibility: 0,
            true_to: 0,
            false_to: 0,
        }
    }

    fn inspect_and_throw(&self, old: u64, divide: bool) -> (u64, usize) {
        let val = self.worry_operand.unwrap_or(old);
        let mut new = if self.multiply { old * val } else { old + val };
        if divide {
            new /= 3;
        }
        let to = if new % self.divisibility == 0 {
            self.true_to
        } else {
            self.false_to
        };
        (new, to)
    }
}

struct Puzzle {
    monkeys: Vec<Monkey>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            monkeys: Vec::new(),
        }
    }

    fn configure(&mut self, path: &str) {
        let mut data = std::fs::read_to_string(path).unwrap();
        data.pop();

        for monkey_configuration in data.split("\n\n") {
            let lines = monkey_configuration.split('\n').collect::<Vec<_>>();
            let mut monkey = Monkey::new();

            monkey.items = lines[1]
                .split_once("Starting items: ")
                .unwrap()
                .1
                .split(", ")
                .map(|item| item.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            monkey.multiply = lines[2].contains('*');
            monkey.worry_operand = lines[2].split(' ').last().unwrap().parse::<u64>().ok();

            monkey.divisibility = lines[3]
                .split_once("Test: divisible by ")
                .unwrap()
                .1
                .parse::<u64>()
                .unwrap();

            monkey.true_to = lines[4]
                .split_once("If true: throw to monkey ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();

            monkey.false_to = lines[5]
                .split_once("If false: throw to monkey ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();

            self.monkeys.push(monkey);
        }
    }

    fn monkey_business(&self, rounds: u32, divide: bool) -> u64 {
        // Make a local copy of monkeys as their items will be modified
        let mut monkeys = self.monkeys.clone();

        // Count of inspections made by monkeys
        let mut inspections: Vec<u64> = Vec::new();
        inspections.resize(self.monkeys.len(), 0);

        // A modulus that keeps item divisibility for every monkey
        let monkeys_modulus = self.monkeys.iter().map(|m| m.divisibility).product::<u64>();

        // Execute the rounds
        for _ in 0..rounds {
            for from in 0..monkeys.len() {
                while !monkeys[from].items.is_empty() {
                    let old = monkeys[from].items.pop().unwrap();
                    let (new, to) = monkeys[from].inspect_and_throw(old, divide);
                    let new = if divide { new } else { new % monkeys_modulus };
                    monkeys[to].items.push(new);
                    inspections[from] += 1;
                }
            }
        }

        // Get the two most active monkeys to compute the monkey business score
        inspections.sort_unstable();
        inspections.pop().unwrap() * inspections.pop().unwrap()
    }

    fn part1(&self) -> u64 {
        self.monkey_business(20, true)
    }

    fn part2(&self) -> u64 {
        self.monkey_business(10000, false)
    }
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test01.txt");
    assert_eq!(puzzle.part1(), 10605);
    assert_eq!(puzzle.part2(), 2713310158);
}

/// Test from user input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), 100345);
    assert_eq!(puzzle.part2(), 28537348205);
}

fn main() {
    let mut puzzle = Puzzle::new();
    let input = std::env::args().nth(1).expect("No input file");
    puzzle.configure(&input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
