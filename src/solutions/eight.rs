use itertools::{Itertools, iproduct};
use std::cmp::max;

fn visible_along(grid: &Vec<&[u8]>, transpose: bool, cols_backwards: bool, rows_backwards: bool) -> Vec<Vec<bool>> {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visibility = vec![vec![false; cols]; rows];

    for i in 0..rows {
        let mut tallest = 0;
        for j in 0..cols {
            let (i, j) = if transpose { (j, i) } else { (i, j) };
            let j = if cols_backwards { cols - j - 1 } else { j };
            let i = if rows_backwards { rows - i - 1 } else { i };
            let tree = grid[i][j];
            visibility[i][j] = tree > tallest;
            tallest = max(tallest, tree);
        }
    }

    visibility
}

fn scenic_score(grid: &Vec<&[u8]>, r: usize, c: usize) -> usize {
    let tree = grid[r][c];

    let mut score = 1;
    for (d_r, d_c) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
        let mut i = 0;
        let (mut r, mut c) = (r, c);
        while let Some(&next) = grid.get((r as isize + d_r) as usize).and_then(|row| row.get((c as isize + d_c) as usize)) {
            i += 1;
            if next >= tree {
                break;
            }
            r = (r as isize + d_r) as usize;
            c = (c as isize + d_c) as usize;
        }
        score *= i;
    }
    score
}

pub fn solve(input: &str) -> (usize, usize) {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let (rows, cols) = (grid.len(), grid[0].len());

    let left = visible_along(&grid, false, false, false);
    let right = visible_along(&grid, false, true, false);
    let up = visible_along(&grid, true, false, false);
    let down = visible_along(&grid, true, true, true);

    let p1 = (0..rows).cartesian_product(0..cols).map(|(r, c)| (left[r][c] || right[r][c] || up[r][c] || down[r][c]) as usize).sum();
    let p2 = (0..rows).cartesian_product(0..cols).map(|(r, c)| scenic_score(&grid, r, c)).max().unwrap();

    (p1, p2)
}