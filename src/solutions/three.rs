use itertools::Itertools;
use std::collections::HashSet;

fn priority(c: char) -> u32 {
    let c = c as u32;
    if c > 96 { c - 96 } else { c - 38 }
}

fn rucksack_priority(rucksack: &str) -> u32 {
    let n = rucksack.len();
    let c1 = &rucksack[..n/2];
    let c2 = &rucksack[n/2..];

    let c1: HashSet<char> = HashSet::from_iter(c1.chars());
    let c2: HashSet<char> = HashSet::from_iter(c2.chars());

    let &overlap = c1.intersection(&c2).collect::<Vec<&char>>()[0];

    priority(overlap)
}

fn group_priority(r1: &str, r2: &str, r3: &str) -> u32 {
    let r1: HashSet<char> = HashSet::from_iter(r1.chars());
    let r2: HashSet<char> = HashSet::from_iter(r2.chars());
    let r3: HashSet<char> = HashSet::from_iter(r3.chars());

    let overlap = [r2, r3].iter().fold(r1, |acc, r| acc.intersection(r).cloned().collect());
    let &badge = overlap.iter().collect::<Vec<&char>>()[0];

    return priority(badge);
}

pub fn solve(input: &str) -> (u32, u32) {
    let rucksacks = input.lines().collect::<Vec<_>>();

    let p1: u32 = rucksacks.iter().map(|r| rucksack_priority(r)).sum();
    let p2: u32 = rucksacks.iter().tuples().map(|(&r1, &r2, &r3)| group_priority(r1, r2, r3)).sum();

    (p1, p2)
}