use itertools::{Itertools, MinMaxResult};
use std::collections::HashSet;

const DIRECTIONS: [[(isize, isize); 3]; 4] = [
    [(-1, 0), (-1, 1), (-1, -1)],
    [(1, 0), (1, 1), (1, -1)],
    [(0, -1), (-1, -1), (1, -1)],
    [(0, 1), (-1, 1), (1, 1)]
];

fn should_move(elves: &HashSet<(isize, isize)>, pos: (isize, isize)) -> bool {
    ![(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)].iter().all(|(dr, dc)| !elves.contains(&(pos.0 + dr, pos.1 + dc)))
}

fn can_move(elves: &HashSet<(isize, isize)>, direction: [(isize, isize); 3], pos: (isize, isize)) -> bool {
    direction.iter().all(|&(dr, dc)| !elves.contains(&(pos.0 + dr, pos.1 + dc)))
}

fn proposal(elves: &HashSet<(isize, isize)>,  directions: &Vec<&[(isize, isize); 3]>, pos: (isize, isize)) -> (isize, isize) {
    if !should_move(elves, pos) { return pos }
    if let Some(proposed) = directions.iter().find(|&&&d| can_move(elves, d, pos)) {
        let (dr, dc) = proposed[0];
        (pos.0 + dr, pos.1 + dc)
    } else {
        pos
    }
}

fn next_round(elves: &HashSet<(isize, isize)>, round: usize) -> HashSet<(isize, isize)> {
    let directions = DIRECTIONS.iter().cycle().dropping(round).take(4).collect_vec();
    let proposals = elves.iter().map(|&p| (p, proposal(&elves, &directions, p))).collect_vec();
    proposals.iter().map(|&(current, proposed)| {
        if proposals.iter().filter(|(_, other)| *other == proposed).count() > 1 { current } else { proposed }
    }).collect()
}

fn exec_rounds(elves: &HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    (0usize..10).fold(elves.clone(), |elf_positions, round| {
        next_round(&elf_positions, round)
    })
}

fn stable(elves: &HashSet<(isize, isize)>) -> usize {
    let mut elf_positions = elves.clone();
    for round in 1.. {
        let next = next_round(&elf_positions, round);
        if next.eq(&elf_positions) { return round; }
        elf_positions = next;
    }
    unreachable!()
}

pub fn solve(input: &str) -> (usize, usize) {
    let board = input.lines().map(|l| l.as_bytes()).collect_vec();
    let (n, m) = (board.len(), board[0].len());
    let elves = (0..n).cartesian_product(0..m).filter(|&(i, j)| board[i][j] == b'#').map(|(i, j)| (i as isize, j as isize)).collect::<HashSet<_>>();

    let elves_moved = exec_rounds(&elves);
    let(r_min, r_max) = match elves_moved.iter().map(|&(r, _)| r).minmax() {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => unreachable!()
    };
    let(c_min, c_max) = match elves_moved.iter().map(|&(_, c)| c).minmax() {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => unreachable!()
    };
    let p1 = (r_max.abs_diff(r_min) + 1) * (c_max.abs_diff(c_min) + 1) - elves_moved.len();
    let p2 = stable(&elves);

    (p1, p2)
}