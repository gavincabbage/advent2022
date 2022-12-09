use std::collections::HashSet;
use std::io::repeat;
use std::iter::repeat_with;
use std::str::{FromStr, Lines};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() {
    let example_file = Data::get("example.txt").unwrap();
    let example_data = std::str::from_utf8(example_file.data.as_ref()).unwrap();
    assert_eq!(13, part1(example_data));
    assert_eq!(1, part2(example_data));

    let example2_file = Data::get("example2.txt").unwrap();
    let example2_data = std::str::from_utf8(example2_file.data.as_ref()).unwrap();
    assert_eq!(36, part2(example2_data));

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!(6357, part1(input_data));
    assert_eq!(2627, part2(input_data));
}

fn part1(data: &str) -> usize {
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let start = Coordinate{ x: 0, y: 0 };
    let mut head = start;
    let mut tail = start;
    visited.insert(start);

    parse(data).iter().for_each(|m| {
        head = move_head(head, *m);
        tail = move_tail(head, tail);
        visited.insert(tail);
    });

    visited.len()
}

fn part2(data: &str) -> usize {
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let start = Coordinate{ x: 0, y: 0 };
    let mut rope: Vec<Coordinate> = std::iter::repeat(start).take(10).collect();
    visited.insert(start);

    parse(data).iter().for_each(|m| {
        rope[0] = move_head(rope[0], *m);
        (1..10).for_each(|ndx| {
            rope[ndx] = move_tail(rope[ndx-1], rope[ndx]);
        });
        visited.insert(rope[9]);
    });

    visited.len()
}

fn parse(data: &str) -> Vec<Direction> {
    let mut moves: Vec<Direction> = Vec::new();
    data.lines().for_each(|line| {
        let split: Vec<&str> = line.split_whitespace().collect();
        let direction= Direction::from_str(split[0]).unwrap();
        let count: usize = split[1].parse().unwrap_or_default();
        (0..count).for_each(|n| { moves.push(direction.clone()) })
    });
    moves
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Direction, ()> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32
}

fn move_head(from: Coordinate, direction: Direction) -> Coordinate {
    match direction {
        Direction::Up => Coordinate{x: from.x, y: from.y+1 },
        Direction::Down => Coordinate{x: from.x, y: from.y - 1 },
        Direction::Left => Coordinate{x: from.x - 1, y: from.y},
        Direction::Right => Coordinate{x: from.x + 1, y: from.y},
    }
}

fn move_tail(head: Coordinate, tail: Coordinate) -> Coordinate {
    let (xd, yd) = (head.x - tail.x, head.y - tail.y);
    let mut new = tail;

    if xd > 1 {
        new.x += 1;
        if yd == 1 || yd == -1 {
            new.y += yd
        }
    } else if xd < -1 {
        new.x -= 1;
        if yd == 1 || yd == -1 {
            new.y += yd
        }
    }

    if yd > 1 {
        new.y += 1;
        if xd == 1 || xd == -1 {
            new.x += xd
        }
    } else if yd < -1 {
        new.y -= 1;
        if xd == 1 || xd == -1 {
            new.x += xd
        }
    }

    new
}
