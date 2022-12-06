use rust_embed::RustEmbed;
use regex::Regex;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() {
    let example_file = Data::get("example.txt").unwrap();
    let example_data = std::str::from_utf8(example_file.data.as_ref()).unwrap();
    assert_eq!("CMZ", part1(example_data));
    assert_eq!("MCD", part2(example_data));

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!("VJSFHWGFT", part1(input_data));
    assert_eq!("LCTQFBVZV", part2(input_data));
}

fn part1(data: &str) -> String {
    let (mut setup, moves) = parse(data);
    setup.apply(moves);
    setup.result()
}

fn part2(data: &str) -> String {
    let (mut setup, moves) = parse(data);
    setup.apply_v2(moves);
    setup.result()
}

#[derive(Debug)]
struct Setup(HashMap<i32, Vec<char>>);

impl Setup {
    fn result(&mut self) -> String {
        let c = self.0.len() as i32;
        (0..c).fold(String::new(), |s, n| {
                s + self.0.get_mut(&(n+1)).unwrap().pop().unwrap_or_default().to_string().as_str()
            })
    }

    fn apply(&mut self, moves: Vec<Move>) {
        moves.into_iter().for_each(|m| {
            (0..m.count).for_each(|_| {
                if let Some(popped) = self.0.get_mut(&m.src).unwrap().pop() {
                    self.0.get_mut(&m.dest).unwrap().push(popped)
                }
            })
        })
    }

    fn apply_v2(&mut self, moves: Vec<Move>) {
        moves.into_iter().for_each(|m| {
            let mut moved = Vec::new();
            (0..m.count).for_each(|_| {
                if let Some(popped) = self.0.get_mut(&m.src).unwrap().pop() {
                    moved.push(popped);
                }
            });
            moved.into_iter().rev().for_each(|x| {
                self.0.get_mut(&m.dest).unwrap().push(x)
            });
        })
    }

}

#[derive(Debug)]
struct Move {
    count: i32,
    src: i32,
    dest: i32,
}

fn parse(data: &str) -> (Setup, Vec<Move>) {
    let sections: Vec<&str> = data.split("\n\n").collect();
    let setup = parse_setup(sections[0]);
    let moves = parse_moves(sections[1]);
    (setup, moves)
}

fn parse_setup(data: &str) -> Setup {
    let lines: Vec<&str> = data.lines().collect();
    let last: Vec<&str> = lines[lines.len()-1].split_whitespace().collect();
    let mut stacks: HashMap<i32, Vec<char>> = HashMap::with_capacity(last.len());
    lines.into_iter().rev().skip(1)
        .for_each(|line| {
        line.chars().enumerate()
            .filter(|&(ndx, l)| ndx % 4 == 1 && l != ' ')
            .for_each(|(ndx, l)| {
                let stack_key = (ndx / 4 + 1) as i32;
                if let Some(stack) = stacks.get_mut(&stack_key) {
                    stack.push(l)
                } else {
                    stacks.insert(stack_key, vec![l]);
                };
            });
    });
    Setup(stacks)
}

fn parse_moves(data: &str) -> Vec<Move> {
    let pattern = Regex::new(r"move (?P<count>\d+) from (?P<src>\d+) to (?P<dest>\d+)").unwrap();
    data.lines().map(|line| {
        let caps = pattern.captures(line).unwrap();
        Move {
            count: caps["count"].parse().unwrap_or_default(),
            src: caps["src"].parse().unwrap_or_default(),
            dest: caps["dest"].parse().unwrap_or_default(),
        }
    }).collect()
}