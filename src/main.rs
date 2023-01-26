use std::fs;

mod solutions;

fn main() {
    let path = "../inputs/23.in";
    let input = fs::read_to_string(path)
        .expect("Error reading input");

    let (p1, p2) = solutions::twentythree::solve(&input);

    println!("p1: {}", p1);
    println!("p2: {}", p2);
}
