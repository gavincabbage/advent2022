use std::fs;

fn main() {
    println!("[Part 1]");
    part1();
    println!("[Part 2]");
    part2();
}

fn part1() {
    let data = fs::read_to_string("./data/calories.txt").unwrap_or_default();
    let lines = data.lines();
    let mut max = 0;
    let mut current_total = 0;
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            if current_total > max {
                max = current_total;
            }
            current_total = 0;
        } else {
            current_total += line.parse::<u64>().unwrap_or_default();
        }
    }
    println!("Calories held by the Elf with the most: {max}");
}

fn part2() {
    let data = fs::read_to_string("./data/calories.txt").unwrap_or_default();
    let lines = data.lines();
    let mut cals = Vec::new();
    let mut current_total = 0;
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            cals.push(current_total);
            current_total = 0;
        } else {
            current_total += line.parse::<u64>().unwrap_or_default();
        }
    }
    cals.sort();
    let total = cals[cals.len()-3..cals.len()].into_iter().sum::<u64>();
    println!("Total calories held by the top three Elves: {total}");
}