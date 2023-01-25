use itertools::Itertools;
use std::collections::HashMap;
use std::path::PathBuf;

const TOTAL_SPACE: usize = 70000000;
const UPDATE_SPACE: usize = 30000000;

fn build_filesystem(commands: &Vec<&str>) -> HashMap<PathBuf, usize> {
    let mut root = HashMap::new();
    let mut path = PathBuf::new();

    for &command in commands {
        match &command[..2] {
            "cd" => {
                let (_, to) = command.trim().split_once(' ').unwrap();
                match to {
                    "/" => {
                        path.clear();
                        path.push("/");
                    },
                    ".." => {
                        path.pop();
                    },
                    _ => {
                        path.push(to);
                    }
                }
            },
            "ls" => {
                let output = command.lines().dropping(1).collect::<Vec<_>>();
                let size: usize = output.iter().filter_map(|&l| {
                    let (size, _) = l.split_once(' ').unwrap();
                    size.parse::<usize>().ok()
                }).sum();
                root.insert(path.clone(), size);
            },
            _ => unreachable!()
        }
    }

    root
}

pub fn solve(input: &str) -> (usize, usize) {
    let commands = input.split("$ ").dropping(1).collect::<Vec<_>>();
    let fs = build_filesystem(&commands);
    let sizes = fs.keys().map(|path| {
        fs.iter().filter_map(|(p, s)| p.to_str().unwrap().contains(path.to_str().unwrap()).then_some(s)).sum::<usize>()
    }).collect::<Vec<_>>();

    let p1: usize = sizes.iter().filter_map(|&s| (s <= 100000).then_some(s)).sum();

    let used_space = fs.values().sum::<usize>();
    let remaining_space = TOTAL_SPACE - used_space;
    let &p2 = sizes.iter().filter(|&s| remaining_space + s > UPDATE_SPACE).sorted().next().unwrap();
    
    (p1, p2)
}