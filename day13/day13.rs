//!
//! [Day 13: Distress Signal](https://adventofcode.com/2022/day/13)
//!

use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq)]
enum Item {
    ListStart,
    ListEnd,
    Integer(u32),
}

#[derive(Clone)]
struct Packet {
    items: Vec<Item>,
}

struct Puzzle {
    packets: Vec<Packet>,
}

fn find_list_end(packet: &Packet, start: usize) -> usize {
    let mut index = start;
    let mut level = 1;

    while level != 0 {
        index += 1;
        match packet.items.get(index) {
            Some(Item::ListStart) => level += 1,
            Some(Item::ListEnd) => level -= 1,
            _ => (),
        }
    }

    index
}

fn compare(first: &Packet, first_i: usize, second: &Packet, second_i: usize) -> Ordering {
    let (left, right) = (&first.items[first_i], &second.items[second_i]);

    if (left, right) == (&Item::ListStart, &Item::ListStart) {
        return compare(first, first_i + 1, second, second_i + 1);
    } else if (left, right) == (&Item::ListEnd, &Item::ListEnd) {
        if first_i == first.items.len() - 1 && second_i == second.items.len() - 1 {
            return Ordering::Equal;
        } else if first_i == first.items.len() - 1 && second_i != second.items.len() - 1 {
            return Ordering::Less;
        } else if first_i != first.items.len() - 1 && second_i == second.items.len() - 1 {
            return Ordering::Greater;
        }
        return compare(first, first_i + 1, second, second_i + 1);
    } else if let (Item::Integer(fvalue), Item::Integer(svalue)) = (left, right) {
        match fvalue.cmp(svalue) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => return compare(first, first_i + 1, second, second_i + 1),
            Ordering::Greater => return Ordering::Greater,
        }
    } else if let (Item::Integer(value), Item::ListStart) = (left, right) {
        let packet = Packet {
            items: vec![Item::ListStart, Item::Integer(*value), Item::ListEnd],
        };
        let index = find_list_end(second, second_i);
        let other = Packet {
            items: second.items[second_i..=index].iter().copied().collect(),
        };
        let order = compare(&packet, 0, &other, 0);
        if order == Ordering::Equal {
            return compare(first, first_i + 1, second, index + 1);
        }
        return order;
    } else if let (Item::ListStart, Item::Integer(value)) = (left, right) {
        let packet = Packet {
            items: vec![Item::ListStart, Item::Integer(*value), Item::ListEnd],
        };
        let index = find_list_end(first, first_i);
        let other = Packet {
            items: first.items[first_i..=index].iter().copied().collect(),
        };
        let order = compare(&other, 0, &packet, 0);
        if order == Ordering::Equal {
            return compare(first, index + 1, second, second_i + 1);
        }
        return order;
    } else if let (Item::Integer(_), Item::ListEnd) = (left, right) {
        return Ordering::Greater;
    } else if let (Item::ListEnd, Item::Integer(_)) = (left, right) {
        return Ordering::Less;
    } else if let (Item::ListStart, Item::ListEnd) = (left, right) {
        return Ordering::Greater;
    } else if let (Item::ListEnd, Item::ListStart) = (left, right) {
        return Ordering::Less;
    }

    panic!("Case not handled");
}

impl Puzzle {
    fn new() -> Self {
        Self {
            packets: Vec::new(),
        }
    }

    fn configure(&mut self, path: &str) {
        let mut data = std::fs::read_to_string(path).unwrap();
        data.pop();

        for couple in data.split("\n\n") {
            for string in couple.split('\n') {
                let mut packet = Packet { items: Vec::new() };
                let mut integer = None;
                for c in string.chars() {
                    match c {
                        '[' => packet.items.push(Item::ListStart),
                        ']' => {
                            if let Some(value) = integer {
                                packet.items.push(Item::Integer(value));
                                integer = None;
                            }
                            packet.items.push(Item::ListEnd);
                        }
                        ',' => {
                            if let Some(value) = integer {
                                packet.items.push(Item::Integer(value));
                                integer = None;
                            }
                        }
                        _ => {
                            integer = if let Some(value) = integer {
                                Some(value * 10 + c.to_digit(10).unwrap())
                            } else {
                                Some(c.to_digit(10).unwrap())
                            }
                        }
                    }
                }
                self.packets.push(packet);
            }
        }
    }

    fn part1(&self) -> usize {
        let mut sum = 0;
        for (index, packet) in self.packets.chunks(2).enumerate() {
            let first = packet.first().unwrap();
            let second = packet.last().unwrap();
            if compare(first, 0, second, 0) != Ordering::Greater {
                sum += index + 1;
            }
        }
        sum
    }

    fn part2(&self) -> usize {
        let mut packets = self.packets.clone();
        packets.sort_unstable_by(|a, b| compare(a, 0, b, 0));
        let divider_two = Packet {
            items: vec![
                Item::ListStart,
                Item::ListStart,
                Item::Integer(2),
                Item::ListEnd,
                Item::ListEnd,
            ],
        };
        let divider_six = Packet {
            items: vec![
                Item::ListStart,
                Item::ListStart,
                Item::Integer(6),
                Item::ListEnd,
                Item::ListEnd,
            ],
        };
        let (mut index_two, mut index_six) = (0, 0);
        for (index, packet) in packets.iter().enumerate() {
            if compare(&divider_two, 0, packet, 0) != Ordering::Greater {
                index_two = index + 1;
                break;
            }
        }
        for (index, packet) in packets.iter().rev().enumerate() {
            if compare(&divider_six, 0, packet, 0) != Ordering::Less {
                index_six = packets.len() - index + 2;
                break;
            }
        }
        index_two * index_six
    }
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test01.txt");
    assert_eq!(puzzle.part1(), 13);
    assert_eq!(puzzle.part2(), 140);
}

/// Test from user input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), 5393);
    assert_eq!(puzzle.part2(), 26712);
}

fn main() {
    let mut puzzle = Puzzle::new();
    let input = std::env::args().nth(1).expect("No input file");
    puzzle.configure(&input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
