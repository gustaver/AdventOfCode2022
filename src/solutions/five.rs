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

fn exec_instructions(instructions: &Vec<(usize, usize, usize)>, rev: bool) -> String {
    let mut stacks = [
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

    for &(n, from, to) in instructions {
        let stack_from = &stacks[from];
        let move_s = stack_from[stack_from.len() - n..].chars();
        let move_s = if rev { move_s.rev().collect::<String>() } else { move_s.collect::<String>() };
        stacks[from] = String::from(&stack_from[..stack_from.len() - n]);
        stacks[to].push_str(&move_s);
    }

    stacks.iter().map(|s| s.chars().last().unwrap()).collect()
}

pub fn solve(input: &str) -> (String, String) {
    let instructions = input.lines().skip(10).map(parse_instruction).collect::<Vec<_>>();

    let p1 = exec_instructions(&instructions, true);
    let p2 = exec_instructions(&instructions, false);

    (p1, p2)
}