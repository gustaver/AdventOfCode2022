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

fn next_heading(state: State, dir: &str) -> State {
    let (heading, pos) = state;
    let d: isize = if dir == "R" { 1 } else { -1 };
    ((heading as isize + d).rem_euclid(4) as usize, pos)
}

fn flip_heading(heading: usize) -> usize {
    (heading + 2).rem_euclid(4)
}

fn on_board(board: &Vec<Vec<u8>>, pos: (isize, isize)) -> bool {
    let (r, c) = pos;
    !r.is_negative() && !c.is_negative() && (r as usize) < board.len() && (c as usize) < board[r as usize].len() && board[r as usize][c as usize] != b' '
}

fn wrap_board(board: &Vec<Vec<u8>>, state: State) -> State {
    let (heading, pos) = state;
    let (dr, dc) = HEADINGS[flip_heading(heading)];
    let mut next = (pos.0 as isize, pos.1 as isize);
    while on_board(board, (next.0 + dr, next.1 + dc)) { next = (next.0 + dr, next.1 + dc) }
    (heading, (next.0 as usize, next.1 as usize))
}

fn wrap_cube(state: State) -> State {
    let (heading, pos) = state;
    let (r, c) = pos;
    match r {
        0..=49 => match c {
            50..=99 => match heading {
                0 => (1, (100 + c , 0)),
                3 => (1, (0, 149 - r)),
                _ => unreachable!()
            },
            100..=149 => match heading {
                0 => (0, (199, c - 100)),
                1 => (3, (149 - r, 99)),
                2 => (3, (c - 50, 99)),
                _ => unreachable!()
            },
            _ => unreachable!()
        },
        50..=99 => match heading {
            1 => (2, (49, r + 50)),
            3 => (0, (100, r - 50)),
            _ => unreachable!()
        },
        100..=149 => match c {
            0..=49 => match heading {
                0 => (1, (50 + c ,50)),
                3 => (1, (149 - r, 50)),
                _ => unreachable!()
            },
            50..=99 => match heading {
                1 => (3, (149 - r, 149)),
                2 => (3, (100 + c, 49)),
                _ => unreachable!()
            },
            _ => unreachable!()
        },
        150..=199 => match heading {
            1 => (0, (149, r - 100)),
            2 => (2, (0, 100 + c)),
            3 => (2, (0, r - 100)),
            _ => unreachable!()
        },
        _ => unreachable!()
    }
}

fn next_pos(board: &Vec<Vec<u8>>, state: State, cube: bool) -> State {
    let (heading, pos) = state;
    let mut next_state = state;
    let (dr, dc) = HEADINGS[heading];
    let next_pos = (pos.0 as isize + dr, pos.1 as isize + dc);
    if !on_board(board, next_pos) {
        next_state = if cube { wrap_cube(state) } else { wrap_board(board, state) };
    } else {
        next_state = (heading, (next_pos.0 as usize, next_pos.1 as usize));
    }
    let (_, (r, c)) = next_state;
    if board[r][c] == b'#' { state } else { next_state }
}

fn next_state(board: &Vec<Vec<u8>>, state: State, dir: &str, steps: usize, cube: bool) -> State {
    let mut state = next_heading(state, dir);
    for _ in 0..steps { state = next_pos(board, state, cube) }
    state
}

fn exec_path(board: &Vec<Vec<u8>>, path: &Vec<(&str, usize)>, cube: bool) -> State {
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