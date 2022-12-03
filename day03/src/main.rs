use rust_embed::RustEmbed;
use itertools::Itertools;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() {
    let example_file = Data::get("example.txt").unwrap();
    let example_data = std::str::from_utf8(example_file.data.as_ref()).unwrap();
    assert_eq!(157, part1(example_data));
    assert_eq!(70, part2(example_data));

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!(7863, part1(input_data));
    assert_eq!(2488, part2(input_data));
}

fn part1(data: &str) -> i32 {
    let lines = data.lines();
    lines.fold(0, | total, line| {
        let first_half = &line[0..line.len()/2];
        let second_half = &line[line.len()/2..line.len()];
        let mut common: char = '_';
        first_half.chars().any(|c| {
            let found = second_half.contains(c);
            common = c;
            found
        });
        total + priority(common)
    })
}

fn part2(data: &str) -> i32 {
    let lines = data.lines();
    lines.chunks(3).into_iter().fold(0, |total, group| {
        let mut common: char = '_';
        let group: Vec<&str> = group.collect();
        group[0].chars().any(|c| {
            let found = group[1].contains(c) && group[2].contains(c);
            common = c;
            found
        });
        total + priority(common)
    })
}

fn priority(char: char) -> i32 {
    ALPHABET.chars().position(|l| l == char).unwrap_or_default() as i32 + 1
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[cfg(test)]
mod tests {
    use crate::priority;

    #[test]
    fn test_priority() {
        let result = priority('a');
        assert_eq!(result, 1);
        let result = priority('z');
        assert_eq!(result, 26);
        let result = priority('A');
        assert_eq!(result, 27);
        let result = priority('Z');
        assert_eq!(result, 52);
    }
}