use std::cmp::{max, Ordering};
use rust_embed::RustEmbed;
use serde_json::{Value};
use serde_json::Value::{Array, Number};

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() {
    let example_file = Data::get("example.txt").unwrap();
    let example_data = std::str::from_utf8(example_file.data.as_ref()).unwrap();
    assert_eq!(13, part1(example_data));
    assert_eq!(140, part2(example_data));

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!(5843, part1(input_data));
    assert_eq!(0, part2(input_data));
}

fn part1(data: &str) -> usize {
    let pairs = data.split("\n\n");
    pairs.enumerate().map(| (n, p)| {
        let pair: Vec<&str> = p.lines().collect();
        let left: Value = serde_json::from_str(pair[0]).unwrap();
        let right: Value = serde_json::from_str(pair[1]).unwrap();
        if in_order(left, right) { n+1 } else { 0 }
    }).sum()
}

fn part2(data: &str) -> usize {
    let mut packets: Vec<Value> = data.lines()
        .filter(|l| *l != "")
        .chain(vec!["[[2]]", "[[6]]"].into_iter())
        .map(|x| serde_json::from_str(x).unwrap())
        .collect();

    packets.sort_unstable_by(sort);

    packets.iter()
        .map(|x| serde_json::to_string(x).unwrap())
        .enumerate()
        .filter(|(_, x)| x == "[[2]]" || x == "[[6]]")
        .map(|(n, _)| n+1)
        .product()
}

fn in_order(left: Value, right: Value) -> bool {
    cmp(left, right) == Ordering::Less
}

fn sort(left: &Value, right: &Value) -> Ordering {
    cmp(left.clone(), right.clone())
}

fn cmp(left: Value, right: Value) -> Ordering {
    match (left, right) {
        (Number(x), Number(y)) => {
            match (x.as_u64(), y.as_u64())  {
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (Some(x), Some(y)) => x.cmp(&y),
                (None, None) => unreachable!(),
            }
        },
        (Array(x), Number(y)) => {
            let a: Vec<Value> = vec![Number(y)];
            cmp(Array(x), Array(a))
        },
        (Number(x), Array(y)) => {
            let a: Vec<Value> = vec![Number(x)];
            cmp(Array(a), Array(y))
        },
        (Array(x), Array(y)) => {
            let max = max(x.len(), y.len());
            let mut result = Ordering::Equal;
            let mut ndx = 0;
            while result == Ordering::Equal && ndx < max {
                result = match (x.get(ndx), y.get(ndx)) {
                    (Some(_), None) => Ordering::Greater,
                    (None, Some(_)) => Ordering::Less,
                    (Some(x), Some(y)) => cmp(x.clone(), y.clone()),
                    (None, None) => unreachable!(),
                };
                ndx += 1;
            }
            result
        },
        _ => unreachable!(),
    }
}