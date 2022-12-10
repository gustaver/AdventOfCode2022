use std::collections::HashSet;

fn adjacent(x: &(isize, isize), y: &(isize, isize)) -> bool {
    x.0.abs_diff(y.0) <= 1 && x.1.abs_diff(y.1) <= 1
}

fn execute_moves(moves: &Vec<(&str, isize)>, rope_len: usize) -> usize {
    let mut rope: Vec<(isize, isize)> = vec![(0, 0); rope_len];
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    for &(dir, len) in moves {
        for _ in 0..len {
            let (d_x, d_y) = match dir {
                "L" => (-1, 0),
                "U" => (0, 1),
                "R" => (1, 0),
                "D" => (0, -1),
                _ => unreachable!()
            };
            rope[0] = (rope[0].0 + d_x, rope[0].1 + d_y);
            for i in 1..rope.len() {
                let knot = rope[i];
                let head = rope[i - 1];
                if !adjacent(&knot, &head) {
                    let (d_x, d_y) = (head.0 - knot.0, head.1 - knot.1);
                    rope[i] = (knot.0 + d_x.signum(), knot.1 + d_y.signum());   
                }
            }
            visited.insert(*rope.last().unwrap());
        }
    }
    
    visited.len()
}

pub fn solve(input: &str) -> (usize, usize) {
    let moves = input.lines().map(|l| {
        let (dir, len) = l.split_once(" ").unwrap();
        (dir, len.parse::<isize>().unwrap())
    }).collect::<Vec<_>>();

    let p1 = execute_moves(&moves, 2);
    let p2 = execute_moves(&moves, 10);

    (p1, p2)
}