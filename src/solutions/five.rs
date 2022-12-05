// [P]     [C]         [M]            
// [D]     [P] [B]     [V] [S]        
// [Q] [V] [R] [V]     [G] [B]        
// [R] [W] [G] [J]     [T] [M]     [V]
// [V] [Q] [Q] [F] [C] [N] [V]     [W]
// [B] [Z] [Z] [H] [L] [P] [L] [J] [N]
// [H] [D] [L] [D] [W] [R] [R] [P] [C]
// [F] [L] [H] [R] [Z] [J] [J] [D] [D]
//  1   2   3   4   5   6   7   8   9

use regex::Regex;
use lazy_static::lazy_static;

fn parse_instruction(line: &str) -> (usize, usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    let captures = RE.captures(line).unwrap();

    (
        captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
        captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1
    )
}

pub fn solve(input: &str) -> (String, String) {
    let stacks = [
        String::from("FHBVRQDP"),
        String::from("LDZQWV"),
        String::from("HLZQGRPC"),
        String::from("RDHFJVB"),
        String::from("ZWLC"),
        String::from("JRPNTGVM"),
        String::from("JRLVMBS"),
        String::from("DPJ"),
        String::from("DCNWV")
    ];

    let instructions = input.lines().skip(10).map(parse_instruction).collect::<Vec<_>>();

    let mut p1_stacks = stacks.clone();
    for (n, from, to) in instructions {
        let stack_from = p1_stacks[from].clone();
        let move_s = &stack_from[stack_from.len() - n..].chars().rev().collect::<String>();
        p1_stacks[from] = String::from(&stack_from[..stack_from.len() - n]);
        p1_stacks[to].push_str(move_s);
    }

    let p1: String = p1_stacks.iter().map(|s| s.chars().last().unwrap()).collect();

    (p1, String::from(""))
}