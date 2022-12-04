use rust_embed::RustEmbed;
use regex::Regex;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() {
    let example_file = Data::get("example.txt").unwrap();
    let example_data = std::str::from_utf8(example_file.data.as_ref()).unwrap();
    assert_eq!(2, part1(example_data));
    assert_eq!(4, part2(example_data));

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!(498, part1(input_data));
    assert_eq!(859, part2(input_data));
}

fn part1(data: &str) -> i32 {
    let pattern = Regex::new(r"(?P<l1>\d+)-(?P<l2>\d+),(?P<r1>\d+)-(?P<r2>\d+)").unwrap();
    pattern.captures_iter(data).fold(0, |total, line| {
        if Match::new(line).fully_overlaps() {
            total + 1
        } else {
            total
        }
    })
}

fn part2(data: &str) -> i32 {
    let pattern = Regex::new(r"(?P<l1>\d+)-(?P<l2>\d+),(?P<r1>\d+)-(?P<r2>\d+)").unwrap();
    pattern.captures_iter(data).fold(0, |total, line| {
        if Match::new(line).overlaps() {
            total + 1
        } else {
            total
        }
    })
}

struct Match {
    l1: i32,
    l2: i32,
    r1: i32,
    r2: i32,
}

impl Match {
    fn new(cap: regex::Captures) -> Match {
        Match{
            l1: cap["l1"].parse().unwrap_or_default(),
            l2: cap["l2"].parse().unwrap_or_default(),
            r1: cap["r1"].parse().unwrap_or_default(),
            r2: cap["r2"].parse().unwrap_or_default(),
        }
    }

    fn fully_overlaps(&self) -> bool {
        return (self.l1 >= self.r1 && self.l2 <= self.r2)
            || (self.r1 >= self.l1 && self.r2 <= self.l2)
    }

    fn overlaps(&self) -> bool {
        return (self.l2 >= self.r1 && self.l1 <= self.r2)
            || (self.r2 >= self.l1 && self.r1 <= self.l2)
    }
}
