use std::iter::zip;

use itertools::Itertools;
use regex::Regex;

const HEADINGS: [(isize, isize); 4] = [
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1)
];

type State = (usize, (usize, usize));

fn next_heading(heading: usize, dir: &str) -> usize {
    let d: isize = if dir == "R" { 1 } else { -1 };
    (heading as isize + d).rem_euclid(4) as usize
}

fn flip_heading(heading: usize) -> usize {
    (heading + 2).rem_euclid(4)
}

fn on_board(board: &Vec<Vec<u8>>, pos: (isize, isize)) -> bool {
    let (r, c) = pos;
    !r.is_negative() && !c.is_negative() && (r as usize) < board.len() && (c as usize) < board[r as usize].len() && board[r as usize][c as usize] != b' '
}

fn wrap_board(board: &Vec<Vec<u8>>, pos: (usize, usize), heading: usize) -> (isize, isize) {
    let (dr, dc) = HEADINGS[flip_heading(heading)];
    let mut next = (pos.0 as isize, pos.1 as isize);
    while on_board(board, (next.0 + dr, next.1 + dc)) { next = (next.0 + dr, next.1 + dc) }
    next
}

fn wrap_cube(pos: (usize, usize), heading: usize) -> (usize, usize) {
    let (r, c) = pos;
    match r {
        0..=49 => match c {
            // 1
            50..=99 => match heading {
                0 => (150 + c , 0),
                3 => (0, 149 - r),
                _ => unreachable!()
            },
            // 2
            100..=149 => match heading {
                0 => (199, c - 50),
                1 => (149 + r ,99),
                2 => (c - 50, 99),
                _ => unreachable!()
            },
            _ => unreachable!()
        },
        // 3
        50..=99 => match heading {
            1 => (49, r + 50),
            3 => (100, r - 50),
            _ => unreachable!()
        },
        100..=149 => match c {
            // 4
            0..=49 => match heading {
                0 => (50 + c ,50),
                3 => (149 - r, 50),
                _ => unreachable!()
            },
            // 5
            50..=99 => match heading {
                1 => (149 - r, 149),
                2 => (100 + c, 49),
                _ => unreachable!()
            },
            _ => unreachable!()
        },
        // 6
        150..=199 => match heading {
            1 => (149, c - 100),
            2 => (0, 100 + c),
            3 => (0, r - 100),
            _ => unreachable!()
        },
        _ => unreachable!()
    }
}

fn next_pos(board: &Vec<Vec<u8>>, pos: (usize, usize), heading: usize, cube: bool) -> (usize, usize) {
    let (dr, dc) = HEADINGS[heading];
    let mut next = (pos.0 as isize + dr, pos.1 as isize + dc);
    if !on_board(board, next) {
        next = if cube {
            let w = wrap_cube(pos, heading);
            (w.0 as isize, w.1 as isize)
        } else {
            wrap_board(board, pos, heading)
        };
    }
    if board[next.0 as usize][next.1 as usize] == b'#' { pos } else { (next.0 as usize, next.1 as usize) }
}

fn next_state(board: &Vec<Vec<u8>>, state: (usize, (usize, usize)), dir: &str, steps: usize, cube: bool) -> (usize, (usize, usize)) {
    let (heading, pos) = state;
    let heading = next_heading(heading, dir);
    let mut pos = pos;
    for _ in 0..steps { pos = next_pos(board, pos, heading, cube) }
    (heading, pos)
}

fn exec_path(board: &Vec<Vec<u8>>, path: &Vec<(&str, usize)>, cube: bool) -> (usize, (usize, usize)) {
    let init_pos = (0, board[0].iter().position(|&x| x == b'.').unwrap());
    path.iter().fold((0, init_pos), |state, &(dir, steps)| {
        next_state(board, state, dir, steps, cube)
    })
}

pub fn solve(input: &str) -> (usize, usize) {
    let (board, path) = input.split_once("\n\n").unwrap();
    let path = format!("R{path}");
    let path = zip(
        Regex::new(r"\d+").unwrap().split(&path).filter_map(|d| (d.len() > 0).then_some(d)),
        Regex::new(r"R|L").unwrap().split(&path).filter_map(|m| m.parse::<usize>().ok())
    ).collect_vec();
    let board = board.lines().map(|l| l.as_bytes().to_vec()).collect_vec();

    let (heading, (r, c)) = exec_path(&board, &path, false);
    let p1 = 1000 * (r + 1) + 4 * (c + 1) + heading - 1;

    let (heading, (r, c)) = exec_path(&board, &path, true);
    let p2 = 1000 * (r + 1) + 4 * (c + 1) + heading - 1;

    (p1, p2)
}