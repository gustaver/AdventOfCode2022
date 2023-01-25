use itertools::Itertools;
use std::cmp::{min, max};

fn sand_next(cave: &mut Vec<Vec<u8>>, sand: (usize, usize), y_max: usize) -> Option<(usize, usize)> {
    let (x, y) = sand;
    if y + 1 > y_max { return None }
    let next_row = &cave[y + 1];
    for x in [x, x - 1, x + 1] {
        if next_row[x] == b'.' { return Some((x, y + 1)); }
    }
    None
}

fn sand(cave: &mut Vec<Vec<u8>>, y_max: usize) -> usize {
    let mut units = 0;
    loop {
        let mut sand = (500, 0);
        while let Some((x, y)) = sand_next(cave, sand, y_max) {
            sand = (x, y);
        }
        if sand == (500, 0) { return units + 1 }
        if sand.1 == y_max { break }
        cave[sand.1][sand.0] = b'o';
        units += 1;
    }
    units
}

fn draw_path(cave: &mut Vec<Vec<u8>>, path: &Vec<(usize, usize)>) {
    for (&(x1, y1), &(x2, y2)) in path.iter().tuple_windows() {
        let (x1, x2) = (min(x1, x2), max(x1, x2));
        let (y1, y2) = (min(y1, y2), max(y1, y2));
        for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
            cave[y][x] = b'#';
        }
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let paths = input.lines().map(|l| l.split(" -> ").map(|p| {
        let(x, y) = p.split_once(',').unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    }).collect_vec()).collect_vec();
    let x_max = paths.iter().flatten().max_by_key(|(x, _)| x).unwrap().0;
    let y_max = paths.iter().flatten().max_by_key(|(_, y)| y).unwrap().1;
    let mut cave = vec![vec![b'.'; x_max + 500]; y_max + 3];
    for path in paths {
        draw_path(&mut cave, &path);
    }

    let p1 = sand(&mut cave, y_max);
    for x in 0..cave[0].len() {
        cave[y_max + 2][x] = b'#';
    }
    let p2 = sand(&mut cave, y_max + 2) + p1;
    (p1, p2)
}