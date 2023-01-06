use std::collections::{HashSet, VecDeque};

use itertools::{Itertools, MinMaxResult};

type Point = (isize, isize, isize);

fn adjacent(p: Point) -> Vec<Point> {
    [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)].iter().map(|d| (p.0 + d.0, p.1 + d.1, p.2 + d.2)).collect_vec()
}

fn exposed(p: Point, cubes: &HashSet<Point>) -> Vec<Point> {
    adjacent(p).into_iter().filter(|a| !cubes.contains(a)).collect_vec()
}

fn in_bounds(p: Point, bounds: (isize, isize)) -> bool {
    [p.0, p.1, p.2].iter().all(|&i| bounds.0 <= i && i <= bounds.1)
} 

fn flood(cubes: &HashSet<Point>, bounds: (isize, isize)) -> usize {
    let mut faces = 0;
    let mut visited: HashSet<Point> = HashSet::new();
    let mut to_visit: VecDeque<Point> = VecDeque::new();
    let corner = (bounds.1, bounds.1, bounds.1);
    to_visit.push_back(corner);

    while let Some(next) = to_visit.pop_front() {
        let neighbors = adjacent(next);
        for &n in neighbors.iter().filter(|&p| in_bounds(*p, bounds)) {
            if visited.contains(&n) { continue }
            if cubes.contains(&n) { faces += 1 }
            else {
                to_visit.push_back(n);
                visited.insert(n);
            }
        }
    }
    faces
}

pub fn solve(input: &str) -> (usize, usize) {
    let cubes: HashSet<Point> = input.lines().map(|l| l.split( ",").map(|s| s.parse::<isize>().unwrap()).collect_tuple().unwrap()).collect();

    let p1: usize = cubes.iter().map(|&p| exposed(p, &cubes).len()).sum();

    let (sq_min, sq_max) = match cubes.iter().map(|c| [c.0, c.1, c.2]).flatten().minmax() {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => unreachable!()
    };

    let p2 = flood(&cubes, (sq_min - 1, sq_max + 1));
    
    (p1, p2)
}