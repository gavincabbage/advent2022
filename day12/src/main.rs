use std::cmp::Reverse;
use std::collections::{HashMap};
use rust_embed::RustEmbed;
use priority_queue::priority_queue::PriorityQueue;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() {
    let example_file = Data::get("example.txt").unwrap();
    let example_data = std::str::from_utf8(example_file.data.as_ref()).unwrap();
    assert_eq!(31, part1(example_data));
    assert_eq!(29, part2(example_data));

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!(456, part1(input_data));
    assert_eq!(454, part2(input_data));
}

fn part1(data: &str) -> usize {
    run(data)
}

fn part2(data: &str) -> usize {
    let data = data.replace("S", "a");
    let indices: Vec<usize> = data.as_bytes().iter().enumerate()
        .filter(|(_, &c)| c == 'a' as u8)
        .map(|(n, _)| n)
        .collect();

    let mut min = usize::MAX;
    indices.iter().for_each(|i| {
        let mut candidate = data.clone();
        candidate.replace_range(i..&(i+1), "S");
        let v = run(candidate.as_str());
        if v < min && v != 0 {
            min = v;
        }
    });

    min
}

fn run(data: &str) -> usize {
    let map = HeightMap::new(data);

    let mut open: PriorityQueue<Position, Reverse<usize>> = PriorityQueue::new();
    open.push(map.start, Reverse(0));

    let mut from: HashMap<Position, Position> = HashMap::new();

    let mut gscore: HashMap<Position, usize> = HashMap::new();
    gscore.insert(map.start, 0);

    let mut fscore: HashMap<Position, usize> = HashMap::new();
    fscore.insert(map.start, map.h(map.start));

    let neighbors = |pos: &Position| -> Vec<Position> {
        let mut c = map.pos(pos);
        if c == 'S' {
            c = 'a';
        }
        let cmp = c as u8 + 1;
        let mut neighbors: Vec<Position> = Vec::new();
        if pos.0 < map.width-1 { neighbors.push((pos.0 + 1, pos.1) as Position) }
        if pos.1 < map.height-1 { neighbors.push((pos.0, pos.1 + 1) as Position) }
        if pos.0 > 0 { neighbors.push((pos.0 - 1, pos.1) as Position) }
        if pos.1 > 0 { neighbors.push((pos.0, pos.1 - 1) as Position) }
        neighbors.into_iter().filter(|p| map.pos(p) as u8 <= cmp).collect()
    };

    while open.len() > 0 {
        let mut current = open.pop().unwrap().0;
        if current == map.goal {
            let mut len: usize = 0;
            while let Some(next) = from.get(&current) {
                current = *next;
                len += 1
            }
            return len
        }

        neighbors(&current).into_iter().for_each(|neighbor| {
            let tentative_gscore: usize = gscore[&current] + 1;
            if tentative_gscore < *gscore.get(&neighbor).unwrap_or(&usize::MAX) {
                from.insert(neighbor, current);
                gscore.insert(neighbor,tentative_gscore);
                fscore.insert(neighbor,tentative_gscore + map.h(neighbor));
                open.push_increase(neighbor, Reverse(tentative_gscore)); // push_decrease
            }
        })
    }

    0
}

type Position = (usize, usize);

struct HeightMap {
    data: String,
    width: usize,
    height: usize,
    start: Position,
    goal: Position,
}

impl HeightMap {
    fn new(data: &str) -> HeightMap {
        let width = data.find('\n').unwrap();
        let height = data.len() / width;
        let data = data.replace("\n", "").to_string();

        let offset = |i: char| -> Position {
            let ndx = data.find(i).unwrap();
            let x = ndx % width;
            let y = ndx / width;
            (x, y)
        };
        let start = offset('S');
        let goal = offset('E');

        HeightMap {
            data: data.to_string(),
            width, height, start, goal
        }
    }

    fn pos(&self, (x, y): &Position) -> char {
        let c = self.data.as_bytes()[y * self.width + x] as char;
        if c == 'E' {
            'z'
        } else if c == 'S' {
            'a'
        } else {
            c
        }
    }

    fn h(&self, pos: Position) -> usize {
        (pos.0).abs_diff(self.goal.0) + (pos.1).abs_diff(self.goal.1)
    }
}


