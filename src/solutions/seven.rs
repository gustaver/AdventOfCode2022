use itertools::Itertools;

struct TreeNode {
    name: String,
    parent: Box<Option<TreeNode>>,
    children: Vec<TreeNode>
}

struct FileNode {
    name: String,
    size: usize
}

enum FileSystem {
    Directory(TreeNode),
    File(FileNode)
}

fn build_filesystem(commands: &Vec<&str>) -> FileSystem {
    let root = FileSystem::Directory(TreeNode{name: String::from("\\"), parent: Box::new(None), children: vec![]});
    let current = &root;
    
    for command in commands {
        match current {
        }
    }

    root
}

pub fn solve(input: &str) -> (usize, usize) {
    let commands = input.split("$ ").dropping(1).collect::<Vec<_>>();

    println!("{:?}", commands);
    (0, 0)
}