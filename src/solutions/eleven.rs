use itertools::Itertools;
use regex::Regex;
use lazy_static::lazy_static;

enum Operation {
    Square,
    Multiply(usize),
    Add(usize)
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    test_true: usize, 
    test_false: usize
}

fn parse_operation(operation: &str) -> Operation {
    let (op, y) = operation.split_once(' ').unwrap();
    match op {
        "+" => Operation::Add(y.parse().unwrap()),
        "*" if y == "old" => Operation::Square,
        "*" => Operation::Multiply(y.parse().unwrap()),
        _ => unreachable!()
    }
}

fn parse_monkey(monkey_str: &str) -> Monkey {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Starting items: (.+)\s+Operation: new = old (.+)\s+Test: divisible by (\d+)\s+If true: throw to monkey (\d+)\s+If false: throw to monkey (\d+)").unwrap();
    }
    let captures = RE.captures(monkey_str).unwrap();
    Monkey {
        items: captures.get(1).unwrap().as_str().split(", ").map(|i| i.parse::<usize>().unwrap()).collect_vec(),
        operation: parse_operation(captures.get(2).unwrap().as_str()),
        test: captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        test_true: captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        test_false: captures.get(5).unwrap().as_str().parse::<usize>().unwrap()
    }
}

fn monkey_business(monkeys: &Vec<Monkey>, rounds: usize, op: impl Fn(usize) -> usize) -> usize {
    let mut items = monkeys.iter().map(|m| m.items.clone()).collect::<Vec<_>>();
    let mut inspections = vec![0usize; monkeys.len()];

    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            inspections[i] += items[i].len();
            while let Some(item) = items[i].pop() {
                let worry_level = match monkey.operation {
                    Operation::Square => op(item * item),
                    Operation::Add(other) => op(item + other),
                    Operation::Multiply(other) => op(item * other) 
                };
                let to_monkey = if worry_level % monkey.test == 0 { monkey.test_true } else { monkey.test_false };
                items[to_monkey].push(worry_level);
            }
        }
    }

    inspections.sort();
    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

pub fn solve(input: &str) -> (usize, usize) {
    let monkeys = input.split("\n\n").map(parse_monkey).collect_vec();

    let modulus: usize = monkeys.iter().map(|m| m.test).product();

    let p1 = monkey_business(&monkeys, 20, |x| (x as f64 / 3.) as usize);
    let p2 = monkey_business(&monkeys, 10000, |x| x % modulus);

    (p1, p2)
}