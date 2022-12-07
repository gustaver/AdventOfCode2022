use itertools::Itertools;
use std::collections::HashMap;
use std::path::PathBuf;

enum FSNode {
    Directory((String, HashMap<String, FSNode>)),
    File((String, usize))
}

fn get_in_filesystem(fs: &HashMap<String, FSNode>) -> FSNode {
    
}

fn build_filesystem(commands: &Vec<&str>) -> HashMap<String, FSNode> {
    let root = HashMap::new();
    let mut path = PathBuf::new();

    for &command in commands {
        println!("command: {}", command);
        println!("path: {}", path.to_str().unwrap());
        match &command[..2] {
            "cd" => {
                let (_, to) = command.trim().split_once(" ").unwrap();
                println!("to: {}", to);
                match to {
                    "/" => {
                        path.clear();
                    },
                    ".." => {
                        path.pop();
                    },
                    _ => {
                        path.push(to);
                    }
                }
            },
            "ls" => {},
            _ => {}
        }
    }

    root
}

pub fn solve(input: &str) -> (usize, usize) {
    let commands = input.split("$ ").dropping(1).collect::<Vec<_>>();
    let fs = build_filesystem(&commands);
    (0, 0)
}