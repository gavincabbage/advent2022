use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

// enum LHS {
//     A
// }

fn main() {
    let example_file = Data::get("example.txt").unwrap();
    let example_data = std::str::from_utf8(example_file.data.as_ref()).unwrap();
    assert_eq!(15, total(example_data));
    assert_eq!(12, total_pt2(example_data));

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!(12679, total(input_data));
    assert_eq!(14470, total_pt2(input_data));
}

fn total(data: &str) -> i32 {
    let lines = data.lines();
    lines.fold(0, | total, line| {
        total + score(line)
    })
}

fn score(line: &str) -> i32 {
    let split: Vec<&str> = line.split_whitespace().collect();
    let lhs = split[0];
    let rhs = split[1];
    match rhs {
        "X" => if lhs == "C" { 7 } else if lhs == "A" { 4 } else { 1 },
        "Y" => if lhs == "A" { 8 } else if lhs == "B" { 5 } else { 2 },
        "Z" => if lhs == "B" { 9 } else if lhs == "C" { 6 } else { 3 },
        _ => 0
    }
}

fn total_pt2(data: &str) -> i32 {
    let lines = data.lines();
    lines.fold(0, | total, line| {
        total + score_pt2(line)
    })
}

fn score_pt2(line: &str) -> i32 {
    let split: Vec<&str> = line.split_whitespace().collect();
    let lhs = split[0];
    let rhs = split[1];
    match lhs {
        "A" => if rhs == "X" { 3 } else if rhs == "Y" { 4 } else { 8 }, // win 2 lose 3 draw 2
        "B" => if rhs == "X" { 1 } else if rhs == "Y" { 5 } else { 9 }, // win 3 lose 1 draw 1
        "C" => if rhs == "X" { 2 } else if rhs == "Y" { 6 } else { 7 }, // win 1 lose 2 draw 3
        _ => 0
    }
}

// LHS
// A Rock
// B Paper
// C Scissors

// RHS pt 1
// X Rock 1
// Y Paper 2
// Z Scissors 3
// Paper -> Rock        Y -> A
// Rock -> Scissors     X -> C
// Scissors -> Paper    Z -> B

// RHS pt 2
// X Lose
// Y Draw
// Z Win