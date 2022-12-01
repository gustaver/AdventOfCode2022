use std::fs;
use regex::Regex;

fn main() {
    let path = "../inputs/01.in";
    let input = fs::read_to_string(path)
        .expect("Error reading input");

    let re_elf = Regex::new(r"\n\n").unwrap();
    let re_pack = Regex::new(r"\n").unwrap();
    let elves = re_elf.split(&input)
        .map(|elf| re_pack.split(elf).map(|s| s.parse::<i32>().unwrap()));

    let elf_calories = elves
        .enumerate()
        .map(|(i, pack)| (i, pack.sum::<i32>()))
        .collect::<Vec<_>>();

    let p1 = elf_calories.iter()
        .max_by_key(|&(_, calories)| calories)
        .unwrap();

    let mut elf_calories_sorted = elf_calories.iter().map(|(_, calories)| calories).collect::<Vec<&i32>>();
    elf_calories_sorted.sort_by_key(|&c| -c);

    let p2: i32 = elf_calories_sorted.iter().copied().take(3).sum();

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
}
