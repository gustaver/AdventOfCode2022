use std::{vec, collections::{HashSet, VecDeque}};

use itertools::Itertools;

type State = Vec<(u8, (isize, isize))>;
type Position = (usize, (isize, isize));

const START: (isize, isize) = (0, 1);
const GOAL: (isize, isize) = (36, 100);
const DIRECTIONS: [(isize, isize); 5] = [
    (0, 0),
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1)
];

fn has_blizzard(blizzards: &State, pos: (isize, isize)) -> bool {
    blizzards.iter().find(|&(_, p)| *p == pos).is_some()
}

fn direction(dir: u8) -> (isize, isize) {
    match dir {
        b'^' => (-1, 0),
        b'>' => (0, 1),
        b'v' => (1, 0),
        b'<' => (0, -1),
        _ => unreachable!()
    }
}

fn next(blizzard: (u8, (isize, isize)), board_dims: (usize, usize)) -> (u8, (isize, isize)) {
    let (r_wall, c_wall) = (board_dims.0 as isize - 1, board_dims.1 as isize - 1);
    let (dir, (r, c)) = blizzard;
    let (dr, dc) = direction(dir);
    let next = (r + dr, c + dc);
    if next.0 == 0 { return (dir, (r_wall - 1, c)) }
    if next.1 == 0 { return (dir, (r, c_wall - 1)) }
    if next.0 == r_wall { return (dir, (1, c)) }
    if next.1 == c_wall { return (dir, (r, 1)) }
    (dir, next)
}

fn next_state(blizzards: &State, board_dims: (usize, usize)) -> State {
    blizzards.iter().map(|&b| next(b, board_dims)).collect()
}

fn in_walls(pos: (isize, isize), board_dims: (usize, usize)) -> bool {
    let (r, c) = pos;
    let (r_wall, c_wall) = (board_dims.0 as isize - 1, board_dims.1 as isize - 1);
    r.is_positive() && c.is_positive() && r < r_wall && c < c_wall
}

fn neighbors(pos: (isize, isize), blizzards: &State, board_dims: (usize, usize)) -> Vec<(isize, isize)> {
    DIRECTIONS.iter().map(|&(dr, dc)| (pos.0 + dr, pos.1 + dc)).filter(|&pos| pos == GOAL || pos == START || (in_walls(pos, board_dims) && !has_blizzard(blizzards, pos))).collect_vec()
}

fn bfs_blizzard(blizzards_t: &Vec<State>, board_dims: (usize, usize), goals: Vec<(isize, isize)>) -> usize {
    let mut total = 0;
    let mut start = START;
    for goal in goals {
        let mut visited: HashSet<Position> = HashSet::new();
        let mut to_visit: VecDeque<Position> = VecDeque::new();
        to_visit.push_back((total, start));
        visited.insert((total, start));
    
        while let Some((t, pos)) = to_visit.pop_front() {
            if pos == goal {
                dbg!(goal);
                total = t;
                start = goal;
                break; 
            }
    
            let neighbors = neighbors(pos, &blizzards_t[t + 1], board_dims);
            for n_pos in neighbors {
                let next = (t + 1, n_pos);
                if !visited.contains(&next) {
                    to_visit.push_back(next);
                    visited.insert(next);
                }
            }
        }
    }
    total
}

pub fn solve(input: &str) -> (usize, usize) {
    let board = input.lines().map(|l| l.as_bytes()).collect_vec();
    let board_dims = (board.len(), board[0].len());
    let (n, m) = board_dims;
    let blizzards = (0..n).cartesian_product(0..m).filter_map(|(i, j)| (board[i][j] != b'#' && board[i][j] != b'.').then_some((board[i][j], (i as isize, j as isize)))).collect_vec();
    let blizzards_time = (1..1000).fold(vec![blizzards], |mut acc, _| {
        let prev = acc.last().unwrap();
        let next = next_state(prev, board_dims);
        acc.push(next);
        acc
    });

    let p1 = bfs_blizzard(&blizzards_time, board_dims, vec![GOAL]);
    let p2 = bfs_blizzard(&blizzards_time, board_dims, vec![GOAL, START, GOAL]);

    (p1, p2)
}