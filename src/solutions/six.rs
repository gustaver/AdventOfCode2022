use itertools::Itertools;

pub fn solve(input: &str) -> (usize, usize) {
    let p1 = input.as_bytes().windows(4).position(|msg| msg.iter().all_unique()).unwrap() + 4;
    let p2 = input.as_bytes().windows(14).position(|msg| msg.iter().all_unique()).unwrap() + 14;

    (p1, p2)
}