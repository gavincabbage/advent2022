use std::collections::HashMap;
use rust_embed::RustEmbed;

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
    assert_eq!(51382025916, part2(input_data));
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
    let mut monkeys: Vec<FactorizedMonkey> = parse2(data);
    for _ in 0..10_000 {
        for mndx in 0..monkeys.len() {
            for indx in 0..monkeys[mndx].items.len() {
                let mut x = monkeys[mndx].items[indx].clone();
                x = (monkeys[mndx].op)(x.clone());
                let dest: u64 = (monkeys[mndx].test)(x.clone());
                monkeys[dest as usize].items.push(x.clone());
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

struct FactorizedMonkey {
    items: Vec<HashMap<u64, u64>>,
    op: Box<dyn Fn(HashMap<u64, u64>) -> HashMap<u64, u64>>,
    test: Box<dyn Fn(HashMap<u64, u64>) -> u64>,
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


fn parse2(data: &str) -> Vec<FactorizedMonkey> {
    let mut monkeys: Vec<FactorizedMonkey> = Vec::new();
    let divisors: &mut Vec<u64> = &mut Vec::new();
    let mut item_lines: Vec<&str> = Vec::new();
    data.split("\n\n").for_each(|monkey| {
        let lines: Vec<&str> = monkey.lines().collect();
        item_lines.push(lines[1]);
        let (divisor, test) = parse_test2(&lines[3..=5]);
        divisors.push(divisor);
        monkeys.push(FactorizedMonkey{
            items: vec![],
            op: parse_op2(lines[2]),
            test: test,
            inspections: 0,
        });
    });

    monkeys.iter_mut().enumerate().for_each(|(n, monkey)| {
        monkey.items = parse_items2(item_lines[n], divisors.to_vec());
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

fn parse_items2(data: &str, divisors: Vec<u64>) -> Vec<HashMap<u64, u64>> {
    let mut items: Vec<HashMap<u64, u64>> = Vec::new();
    data.strip_prefix("  Starting items: ").unwrap().split(",").for_each(|s| {
        let n: u64 = s.trim().parse().unwrap();
        let mut item: HashMap<u64, u64> = HashMap::new();
        divisors.iter().for_each(|divisor| {
            item.insert(*divisor, n % divisor);
        });
        items.push(item);
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

fn parse_op2(data: &str) ->  Box<dyn Fn(HashMap<u64, u64>) -> HashMap<u64, u64>> {
    let split: Vec<&str> = data.split_whitespace().rev().take(2).collect();
    match split[0].trim().parse::<u64>() {
        Ok(i) => {
            if split[1] == "+" {
                Box::new(move |m: HashMap<u64, u64>| -> HashMap<u64, u64> {
                    let mut m = m.clone();
                    m.iter_mut().for_each(|(k, v)| {
                        *v = (*v + i) % k
                    });
                    m
                })
            } else {
                Box::new(move |m: HashMap<u64, u64>| -> HashMap<u64, u64> {
                    let mut m = m.clone();
                    m.iter_mut().for_each(|(k, v)| {
                        if *v != 0 {
                            *v = ((*v + k) * i) % k
                        }
                    });
                    m
                })
            }
        },
        _ => Box::new(move |m: HashMap<u64, u64>| -> HashMap<u64, u64> {
            let mut m = m.clone();
            m.iter_mut().for_each(|(k, v)| {
                if *v != 0 {
                    *v = (*v + k).pow(2) % k
                }
            });
            m
        })

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

fn parse_test2(data: &[&str]) ->  (u64, Box<dyn Fn(HashMap<u64, u64>) -> u64>) {
    let get_last = |s: &str| {
        s.split_whitespace().last().unwrap().parse().unwrap()
    };
    let divisor: u64 = get_last(data[0]);
    let td: u64 = get_last(data[1]);
    let fd: u64 = get_last(data[2]);
    (divisor, Box::new(move |m: HashMap<u64, u64>| -> u64 {
        if *m.get(&divisor).unwrap() == 0 {
            td
        } else {
            fd
        }
    }))
}
