use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use regex::Regex;
use lazy_static::lazy_static;

fn parse_line(line: &str) -> (&str, usize, Vec<&str>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    }
    let captures = RE.captures(line).unwrap();
    (
        captures.get(1).unwrap().as_str(),
        captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(3).unwrap().as_str().split(", ").collect_vec()
    )
}

fn max_pressure(flow: &HashMap<&str, usize>, reduced: &HashMap<&str, Vec<(&str, usize)>>, open: HashSet<&str>, current: &str, time: isize, pressure: usize) -> usize {
    if open.len() == flow.len() { return pressure }
    let mut options = vec![];
    for &(next, t) in reduced.get(current).unwrap() {
        let remaining = time - t as isize - 1;
        if remaining >= 0 && !open.contains(next) {
            let mut open_next = open.clone();
            open_next.insert(next);
            options.push(max_pressure(flow, reduced, open_next, next, remaining, pressure + flow.get(next).unwrap() * remaining as usize));
        }
    }
    if options.is_empty() { return pressure }
    *options.iter().max().unwrap()
}

fn reduced_neighbors<'a>(flow: &'a HashMap<&str, usize>, tunnels: &'a HashMap<&str, Vec<&str>>, start: &'a str) -> Vec<(&'a str, usize)> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut to_visit: VecDeque<(&str, usize)> = VecDeque::new();
    to_visit.push_back((start, 0));
    visited.insert(start);

    let mut neighbors: Vec<(&str, usize)> = Vec::new();
    while let Some((current, distance)) = to_visit.pop_front() {
        if current != start && (*flow.get(current).unwrap() > 0 || current == "AA") { neighbors.push((current, distance)) }
        for n in tunnels.get(current).unwrap() {
            if visited.contains(n) { continue }
            to_visit.push_back((n, distance + 1));
            visited.insert(n);
        }
    }

    neighbors
}

pub fn solve(input: &str) -> (usize, usize) {
    let parsed = input.lines().map(|l| parse_line(l));
    let flow = parsed.clone().map(|p| (p.0, p.1)).collect::<HashMap<_, _>>();
    let tunnels = parsed.map(|p| (p.0, p.2)).collect::<HashMap<_, _>>();
    let not_jammed = flow.iter().filter(|&(v, f)| *f > 0 || *v == "AA");
    let reduced = not_jammed.map(|(valve, _)| (*valve, reduced_neighbors(&flow, &tunnels, valve))).collect::<HashMap<_,_>>();

    let open = flow.iter().filter_map(|(v, f)| (*f == 0).then_some(*v)).collect::<HashSet<_>>();
    let p1 = max_pressure(&flow, &reduced, open.clone(), "AA", 30, 0);

    let to_open = flow.iter().filter_map(|(v, f)| (*f != 0).then_some(*v)).collect::<HashSet<_>>();
    let p2 = to_open.iter().combinations(to_open.len() / 2 as usize).map(|half_open| {
        let half_open = half_open.iter().map(|v| **v).collect::<HashSet<_>>();
        let other_half = to_open.difference(&half_open).map(|&s| s).collect::<HashSet<_>>();

        let you_open = open.union(&half_open).map(|&s| s).collect::<HashSet<_>>();
        let elephant_open = open.union(&other_half).map(|&s| s).collect::<HashSet<_>>();
        max_pressure(&flow, &reduced, you_open, "AA", 26, 0) + max_pressure(&flow, &reduced, elephant_open, "AA", 26, 0)

    }).max().unwrap();

    (p1, p2)
}