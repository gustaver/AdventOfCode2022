use std::collections::{HashSet, HashMap};

use itertools::Itertools;

const ROCK_1: [[bool; 4];4] = [
    [false, false, false, false],
    [false, false, false, false],
    [false, false, false, false],
    [true, true, true, true],
];
const ROCK_2: [[bool; 4];4] = [
    [false, false, false, false],
    [false, true, false, false],
    [true, true, true, false],
    [false, true, false, false],
];
const ROCK_3: [[bool; 4];4] = [
    [false, false, false, false],
    [false, false, true, false],
    [false, false, true, false],
    [true, true, true, false],
];
const ROCK_4: [[bool; 4];4] = [
    [true, false, false, false],
    [true, false, false, false],
    [true, false, false, false],
    [true, false, false, false],
];
const ROCK_5: [[bool; 4];4] = [
    [false, false, false, false],
    [false, false, false, false],
    [true, true, false, false],
    [true, true, false, false],
];

enum HeightOrCycle {
    Height(usize),
    Cycle((usize, usize, Vec<usize>))
}

fn collision(cave: &Vec<Vec<bool>>, rock: [[bool; 4]; 4], position: (usize, usize)) -> bool {
    (0..4usize).cartesian_product(0..4usize).filter(|&(i, j)| rock[i][j]).any(|(i, j)| cave[position.0 - i][position.1 + j])
}

fn height(cave: &Vec<Vec<bool>>) -> usize {
    cave.iter().position(|r| r[1..=7].iter().all(|b| !b)).unwrap() - 1
}

fn draw_rock(cave: &mut Vec<Vec<bool>>, rock: [[bool; 4]; 4], position: (usize, usize)) {
    for (i, j) in (0..4usize).cartesian_product(0..4usize).filter(|&(i, j)| rock[i][j]) {
        let (x, y) = (position.0 - i, position.1 + j);
        cave[x][y] = rock[i][j] || cave[x][y];
    }
}

fn simulate(jets: &Vec<isize>, n: usize) -> HeightOrCycle {
    let rocks = [ROCK_1, ROCK_2, ROCK_3, ROCK_4, ROCK_5];
    let mut states: Vec<(usize, usize, usize)> = Vec::new();
    let mut cave = vec![vec![false; 9]; n * 4];
    (0..9).for_each(|j| cave[0][j] = true);
    cave.iter_mut().for_each(|r| {
        r[0] = true;
        r[8] = true;
    });

    let mut rock_iter = rocks.iter().enumerate().cycle();
    let mut jet_iter = jets.iter().enumerate().cycle().peekable();
    for i in 1..=n {
        let (ri, &rock) = rock_iter.next().unwrap();
        let ji = jet_iter.peek().unwrap().0;
        let h = height(&cave);

        let state = (ri, ji, h);
        states.push(state);
        let maybe_cycle = states.iter().enumerate().filter(|&(_, &(r, j, _))| (r, j) == (ri, ji)).tuple_windows().find(|(a, b, c)| {
            b.1.1 - a.1.1 == c.1.1 - b.1.1
        });
        if maybe_cycle.is_some() {
            let (_, (start, _), (end, _)) = maybe_cycle.unwrap();
            let cycle = states[start..=end].iter().tuple_windows().map(|(a, b)| b.2 - a.2).collect_vec();
            return HeightOrCycle::Cycle((h, i - 1, cycle))
        }
        
        let mut position = (h + 7, 3);
        loop {
            let dx = jet_iter.next().unwrap().1;
            let x = position.1 as isize + dx;
            if !collision(&cave, rock, (position.0, x as usize)) { position.1 = x as usize }
            if collision(&cave, rock, (position.0 - 1, position.1)) { break }
            position.0 -= 1;
        }
        draw_rock(&mut cave, rock, position);
    }
    HeightOrCycle::Height(height(&cave))
}

pub fn solve(input: &str) -> (usize, usize) {
    let jets: Vec<isize> = input.chars().map(|c| match c {
        '<' => -1,
        '>' => 1,
        _ => unreachable!()
    }).collect_vec();

    let p1 = match simulate(&jets, 2022) {
        HeightOrCycle::Height(h) => h,
        _ => unreachable!()
    };
    let p2 = match simulate(&jets, 10_000) {
        HeightOrCycle::Cycle((h, i, cycle)) => {
            let dh: usize = cycle.iter().sum();
            let rem = 1000000000000 - i;
            let n_cycles = rem / cycle.len() as usize;
            let gap = rem % cycle.len();
            h + dh * n_cycles + cycle[..gap].iter().sum::<usize>()
        },
        _ => unreachable!()
    };

    (p1, p2)
}