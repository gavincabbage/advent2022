use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

const SOURCE: (usize, usize) = (500,0);

fn main() {
    let example_file = Data::get("example.txt").unwrap();
    let example_data = std::str::from_utf8(example_file.data.as_ref()).unwrap();
    assert_eq!(24, part1(example_data));

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!(0, part1(input_data));
}

fn part1(data: &str) -> usize {





    0
}

struct State {
    solids: Vec<(usize, usize)>
}