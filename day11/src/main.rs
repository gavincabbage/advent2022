use rust_embed::RustEmbed;
use primes::factors_uniq;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() {
    let example_file = Data::get("example.txt").unwrap();
    let example_data = std::str::from_utf8(example_file.data.as_ref()).unwrap();
    assert_eq!(10605, part1(example_data));
    assert_eq!(2713310158, part2(example_data));

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!(151312, part1(input_data));
    assert_eq!(0, part2(input_data));
}

fn part1(data: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = parse(data);
    for _ in 0..20 {
        for mndx in 0..monkeys.len() {
            for indx in 0..monkeys[mndx].items.len() {
                let mut x = monkeys[mndx].items[indx];
                x = (monkeys[mndx].op)(x);
                x = x / 3;
                let dest: u64 = (monkeys[mndx].test)(x);
                monkeys[dest as usize].items.push(x);
                monkeys[mndx].inspections += 1;
            }
            monkeys[mndx].items = Vec::new();
        }
    }

    monkeys.sort_unstable_by(|a, b| b.inspections.cmp(&a.inspections));
    monkeys.iter().take(2).fold(1, |x, monkey| {
        x * monkey.inspections
    })
}

fn part2(data: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = parse2(data);
    for _ in 0..10_000 {
        for mndx in 0..monkeys.len() {
            for indx in 0..monkeys[mndx].items.len() {
                let mut x = monkeys[mndx].items[indx];
                x = (monkeys[mndx].op)(x);
                let dest: u64 = (monkeys[mndx].test)(x);
                // x = reduce(x);
                monkeys[dest as usize].items.push(x);
                monkeys[mndx].inspections += 1;
            }
            monkeys[mndx].items = Vec::new();
        }
    }

    monkeys.sort_unstable_by(|a, b| b.inspections.cmp(&a.inspections));
    monkeys.iter().take(2).fold(1, |x, monkey| {
        x * monkey.inspections
    })
}

struct Monkey {
    items: Vec<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> u64>,
    inspections: u64,
}

fn parse(data: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    data.split("\n\n").for_each(|monkey| {
        let lines: Vec<&str> = monkey.lines().collect();
        monkeys.push(Monkey{
            items: parse_items(lines[1]),
            op: parse_op(lines[2]),
            test: parse_test(&lines[3..=5]),
            inspections: 0,
        });
    });
    monkeys
}


fn parse2(data: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    data.split("\n\n").for_each(|monkey| {
        let lines: Vec<&str> = monkey.lines().collect();
        monkeys.push(Monkey{
            items: parse_items(lines[1]),
            op: parse_op2(lines[2]),
            test: parse_test(&lines[3..=5]),
            inspections: 0,
        });
    });
    monkeys
}

fn parse_items(data: &str) -> Vec<u64> {
    let mut items: Vec<u64> = Vec::new();
    data.strip_prefix("  Starting items: ").unwrap().split(",").for_each(|s| {
        items.push(s.trim().parse().unwrap());
    });
    items
}

fn parse_op(data: &str) ->  Box<dyn Fn(u64) -> u64> {
    let split: Vec<&str> = data.split_whitespace().rev().take(2).collect();
    match split[0].trim().parse::<u64>() {
        Ok(i) => {
            if split[1] == "+" {
                Box::new(move |n: u64| -> u64 { n + i })
            } else {
                Box::new(move |n: u64| -> u64 { n * i })
            }
        },
        _ => Box::new(move |n: u64| -> u64 { n * n })
    }
}

fn parse_op2(data: &str) ->  Box<dyn Fn(u64) -> u64> {
    let split: Vec<&str> = data.split_whitespace().rev().take(2).collect();
    match split[0].trim().parse::<u64>() {
        Ok(i) => {
            if split[1] == "+" {
                Box::new(move |n: u64| -> u64 { n + i })
            } else {
                Box::new(move |n: u64| -> u64 { n + (n % i) })
            }
        },
        _ => Box::new(move |n: u64| -> u64 { n })
    }
}

fn parse_test(data: &[&str]) ->  Box<dyn Fn(u64) -> u64> {
    let get_last = |s: &str| {
        s.split_whitespace().last().unwrap().parse().unwrap()
    };
    let divisor: u64 = get_last(data[0]);
    let td: u64 = get_last(data[1]);
    let fd: u64 = get_last(data[2]);
    Box::new(move |n: u64| -> u64 {
        if n % divisor == 0 {
            td
        } else {
            fd
        }
    })
}

fn reduce(x: u64) -> u64 {
    let r = factors_uniq(x).iter().fold(1, |i, f| if f < &1000 { i * f } else { i });
    println!("{} {} {:?}", x, r, factors_uniq(x));
    r
}
