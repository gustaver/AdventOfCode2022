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

// fn in_bounds(board_dims: (usize, usize), pos: (isize, isize)) -> bool {
//     let (n, m) = board_dims;
//     let (r, c) = pos;
//     0 <= r && r < n as isize && 0 <= c && c < m as isize
// }

fn can_move(elves: &HashSet<(isize, isize)>, board_dims: (usize, usize), direction: [(isize, isize); 3], pos: (isize, isize)) -> bool {
    direction.iter().all(|&(dr, dc)| {
        let next = (pos.0 + dr, pos.1 + dc);
        // in_bounds(board_dims, next) && 
        !elves.contains(&next)
    })
}

fn proposal(elves: &HashSet<(isize, isize)>, board_dims: (usize, usize), directions: &Vec<&[(isize, isize); 3]>, pos: (isize, isize)) -> (isize, isize) {
    if !should_move(elves, pos) { return pos }
    if let Some(proposed) = directions.iter().find(|&&&d| can_move(elves, board_dims, d, pos)) {
        let (dr, dc) = proposed[0];
        (pos.0 + dr, pos.1 + dc)
    } else {
        pos
    }
}

fn exec_rounds(elves: &HashSet<(isize, isize)>, board_dims: (usize, usize)) -> HashSet<(isize, isize)> {
    (0usize..=10).fold(elves.clone(), |elf_positions, round| {
        // dbg!(elf_positions.iter().filter(|&&p| in_bounds(board_dims, p)).count());
        // dbg!(elf_positions.len());
        let directions = DIRECTIONS.iter().cycle().dropping(round).take(4).collect_vec();
        let proposals = elf_positions.iter().map(|&p| (p, proposal(&elf_positions, board_dims, &directions, p))).collect_vec();
        proposals.iter().map(|&(current, proposed)| {
            if proposals.iter().filter(|(_, other)| *other == proposed).count() > 1 { current } else { proposed }
        }).collect()
    })
}

pub fn solve(input: &str) -> (usize, usize) {
    let board = input.lines().map(|l| l.as_bytes()).collect_vec();
    let (n, m) = (board.len(), board[0].len());
    let board_dims = (n, m);
    let elves = (0..n).cartesian_product(0..m).filter(|&(i, j)| board[i][j] == b'#').map(|(i, j)| (i as isize, j as isize)).collect::<HashSet<_>>();

    // dbg!(can_move(&elves, board_dims, DIRECTIONS[0], (7, 5)));
    // dbg!(should_move(&elves, (73, 19)));
    // let directions = DIRECTIONS.iter().cycle().dropping(0).take(4).collect_vec();
    // dbg!(proposal(&elves, board_dims, &directions, (73, 20)));

    let elves_moved = exec_rounds(&elves, (n, m));
    // dbg!(elves_moved.len() == elves.len());
    let(r_min, r_max) = match elves_moved.iter().map(|&(r, _)| r).minmax() {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => unreachable!()
    };
    let(c_min, c_max) = match elves_moved.iter().map(|&(_, c)| c).minmax() {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => unreachable!()
    };
    let p1 = (r_max.abs_diff(r_min) + 1) * (c_max.abs_diff(c_min) + 1) - elves_moved.len();
    let p2 = 0;

    (p1, p2)
}