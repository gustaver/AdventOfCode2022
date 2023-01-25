use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn get_pos(board: &Vec<Vec<u8>>, e: u8) -> Vec<(usize, usize)> {
    (0..board.len()).cartesian_product(0..board[0].len()).filter(|&(x, y)| board[x][y] == e).collect_vec()
}

fn neighbors(board: &Vec<Vec<u8>>, pos: (usize, usize), visited: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let h = board[pos.0][pos.1];
    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let (x, y) = ((pos.0 as isize + dx) as usize, (pos.1 as isize + dy) as usize);
        if let Some(&n) = board.get(x).and_then(|row| row.get(y)) {
            if h + 1 >= n && !visited.contains(&(x, y)) { neighbors.push((x, y)) }
        }
    }
    neighbors
}

fn bfs_shortest_path(board: &Vec<Vec<u8>>, start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit: VecDeque<((usize, usize), usize)> = VecDeque::new();
    to_visit.push_back((start, 0));
    visited.insert(start);

    while let Some((next, distance)) = to_visit.pop_front() {
        if next == goal {
            return Some(distance);
        }

        let neighbors = neighbors(board, next, &visited);
        for n in neighbors {
            to_visit.push_back((n, distance + 1));
            visited.insert(n);

        }
    }
    None
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut board = input.lines().map(|l| l.as_bytes().iter().copied().collect_vec()).collect_vec();

    let start = get_pos(&board, b'S')[0];
    let goal = get_pos(&board, b'E')[0];

    board[start.0][start.1] = b'a';
    board[goal.0][goal.1] = b'z';

    let p1 = bfs_shortest_path(&board, start, goal);
    let p2 = get_pos(&board, b'a').iter()
        .filter_map(|&pos| bfs_shortest_path(&board, pos, goal))
        .min();
    
    (p1.unwrap(), p2.unwrap())
}