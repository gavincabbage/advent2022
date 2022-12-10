use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() {
    let example_file = Data::get("example.txt").unwrap();
    let example_data = std::str::from_utf8(example_file.data.as_ref()).unwrap();
    assert_eq!(13140, part1(example_data));
    part2(example_data);

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!(14520, part1(input_data));
    part2(input_data);
}

fn part1(data: &str) -> i32 {
    let instructions = parse(data);
    let mut total: i32 = 0;
    let mut val: i32 = 1;
    instructions.iter().enumerate().for_each(|(n, op)| {
        let cycle = n+1;
        if cycle == 20 || (cycle+20) % 40 == 0 {
            total += val * cycle as i32;
        }
        val += op;
    });
    total
}

fn part2(data: &str) {
    let instructions = parse(data);
    let mut val: i32 = 1;
    let mut line: usize = 0;
    instructions.iter().enumerate().for_each(|(n, op)| {
        let ndx = (n - (line*40)) as i32;
        if ndx >= val-1 && ndx <= val+1 {
            print!("#");
        } else {
            print!(".");
        }
        val += op;
        if n % 40 == 39 {
            line += 1;
            print!("\n");
        }
    });
    print!("\n");
}

fn parse(data: &str) -> Vec<i32> {
    let mut instructions: Vec<i32> = Vec::new();
    data.lines().for_each(|line| {
        let split: Vec<&str> = line.split_whitespace().collect();
        instructions.push(0);
        if split[0].trim() != "noop" {
            instructions.push(split[1].parse().unwrap_or_default());
        };
    });
    instructions
}