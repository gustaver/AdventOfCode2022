use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use lazy_static::lazy_static;

fn exec_op(op: &str, lhs: f64, rhs: f64) -> f64 {
    match op {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "*" => lhs * rhs,
        "/" => lhs / rhs,
        _ => unreachable!()
    }
}

fn solve_op(op: &str, lhs: Option<f64>, rhs: Option<f64>, result: f64) -> f64 {
    match op {
        "+" => match (lhs, rhs) {
            (Some(l), None) => result - l,
            (None, Some(r)) => result - r,
            _ => unreachable!()
        },
        "-" => match (lhs, rhs) {
            (Some(l), None) => l - result,
            (None, Some(r)) => result + r,
            _ => unreachable!()
        },
        "*" => match (lhs, rhs) {
            (Some(l), None) => result / l,
            (None, Some(r)) => result / r,
            _ => unreachable!()
        },
        "/" => match (lhs, rhs) {
            (Some(l), None) => l / result,
            (None, Some(r)) => result * r,
            _ => unreachable!()
        },
        _ => unreachable!()
    }
}

fn parse_op(op: &str) -> (&str, &str, &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z]{4}) (\+|-|\*|/) ([a-z]{4})").unwrap();
    }
    let captures = RE.captures(op).unwrap();
    captures.iter().dropping(1).map(|c| c.unwrap().as_str()).collect_tuple().unwrap()
}

fn calculate(monkeys: &HashMap<&str, &str>, monkey: &str) -> f64 {
    let yell = monkeys[monkey];
    match yell.parse::<f64>() {
        Ok(x) => x,
        Err(_) => {
            let (lhs, op, rhs) = parse_op(yell);
            exec_op(op, calculate(&monkeys, lhs), calculate(&monkeys, rhs))
        }
    }
}

fn maybe_calculate(monkeys: &HashMap<&str, &str>, monkey: &str) -> Option<f64> {
    if monkey == "humn" { return None }
    let yell = monkeys[monkey];
    match yell.parse::<f64>() {
        Ok(x) => Some(x),
        Err(_) => {
            let (lhs, op, rhs) = parse_op(yell);
            match (maybe_calculate(&monkeys, lhs), maybe_calculate(&monkeys, rhs)) {
                (None, Some(_)) => None,
                (Some(_), None) => None,
                (Some(x), Some(y)) => Some(exec_op(op, x, y)),
                (None, None) => unreachable!()
            }
        }
    }
}

fn humn_equals(monkeys: &HashMap<&str, &str>, monkey: &str, result: f64) -> f64 {
    if monkey == "humn" { return result } 
    let yell = monkeys[monkey];
    match yell.parse::<f64>() {
        Ok(x) => x,
        Err(_) => {
            let (lhs, op, rhs) = parse_op(yell);
            match (maybe_calculate(&monkeys, lhs), maybe_calculate(&monkeys, rhs)) {
                (Some(x), None) => humn_equals(&monkeys, rhs, solve_op(op, Some(x), None, result)),
                (None, Some(x)) => humn_equals(&monkeys, lhs, solve_op(op, None, Some(x), result)),
                _ => unreachable!()    
            }
        }
    }
}

pub fn solve(input: &str) -> (f64, f64) {
    let monkeys = input.lines().map(|l| l.split(": ").collect_tuple().unwrap()).collect::<HashMap<&str, &str>>();

    let p1 = calculate(&monkeys, "root");

    let(lhs, _, rhs) = parse_op(monkeys["root"]);
    let p2 = match (maybe_calculate(&monkeys, lhs), maybe_calculate(&monkeys, rhs)) {
        (Some(x), None) => humn_equals(&monkeys, rhs, x),
        (None, Some(x)) => humn_equals(&monkeys, lhs, x),
        _ => unreachable!()
    };

    (p1, p2)
}