use itertools::Itertools;

pub fn solve(input: &str) -> (i32, i32) {
    let calories = input.split("\n\n")
        .map(|s| s.lines().map(|c| c.parse::<i32>().unwrap()).sum::<i32>())
        .sorted()
        .rev()
        .collect::<Vec<_>>();

    let p1 = calories[0];
    let p2: i32 = calories[0..3].iter().sum();

    (p1, p2)
}