use std::collections::HashSet;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() {
    assert_eq!(7, part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(10, part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(11, part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));

    assert_eq!(19, part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(23, part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(23, part2("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(29, part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(26, part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!(1651, part1(input_data));
    assert_eq!(3837, part2(input_data));
}

fn part1(data: &str) -> usize {
    let mut pos = 0;
    data.chars().enumerate().skip(3).any(|(ndx, _c)|  {
        let set: HashSet<char> = HashSet::from_iter(data[ndx-3..ndx+1].chars().into_iter());
        if set.len() == 4 {
            pos = ndx + 1;
            true
        } else {
            false
        }
    });
    pos
}

fn part2(data: &str) -> usize {
    let mut pos = 0;
    data.chars().enumerate().skip(13).any(|(ndx, _c)|  {
        let set: HashSet<char> = HashSet::from_iter(data[ndx-13..ndx+1].chars().into_iter());
        if set.len() == 14 {
            pos = ndx + 1;
            true
        } else {
            false
        }
    });
    pos
}