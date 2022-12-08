use itertools::Itertools;
use std::collections::HashMap;
use std::path::PathBuf;

enum FSNode {
    Directory(HashMap<String, FSNode>),
    File(usize)
}

fn build_filesystem(commands: &Vec<&str>) -> HashMap<String, FSNode> {
    let mut root = HashMap::new();
    root.insert(String::from("/"), FSNode::Directory(HashMap::new()));

    let mut path = PathBuf::new();

    for &command in commands {
        match &command[..2] {
            "cd" => {
                let (_, to) = command.trim().split_once(" ").unwrap();
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
                let current = &root;
                println!("path: {:?}", path.to_str().unwrap());
                for dir in path.iter() {
                    println!("{:?}", dir);
                    let current = match current.get(dir.to_str().unwrap()).unwrap() {
                        FSNode::Directory(map) => map,
                        _ => unreachable!()
                    };
                }
            },
            _ => unreachable!()
        }
    }

    root
}

pub fn solve(input: &str) -> (usize, usize) {
    let commands = input.split("$ ").dropping(1).collect::<Vec<_>>();
    let fs = build_filesystem(&commands);
    (0, 0)
}