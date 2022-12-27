use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;
use lazy_static::lazy_static;

fn parse_line(line: &str) -> ((isize, isize), (isize, isize)) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    }
    let captures = RE.captures(line).unwrap();
    (
        (captures.get(1).unwrap().as_str().parse::<isize>().unwrap(),
        captures.get(2).unwrap().as_str().parse::<isize>().unwrap()),
        (captures.get(3).unwrap().as_str().parse::<isize>().unwrap(),
        captures.get(4).unwrap().as_str().parse::<isize>().unwrap())
    )
}

fn manhattan(a: (isize, isize), b: (isize, isize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn perimeter(sensor: (isize, isize), nearest_beacon: (isize, isize)) -> Vec<(isize, isize)> {
    let d = manhattan(sensor, nearest_beacon) as isize + 1;
    let mut points = Vec::new();
    for (&a, &b) in [(sensor.0 - d, sensor.1), (sensor.0, sensor.1 + d), (sensor.0 + d, sensor.1), (sensor.0, sensor.1 - d), (sensor.0 - d, sensor.1)].iter().tuple_windows() {
        let c = (b.0 - a.0, b.1 - a.1);
        let (dx, dy) = (c.0.signum(), c.1.signum());
        let mut p = a;
        while p != b {
            if p.0 <= 4000000 && p.0 >= 0 && p.1 <= 4000000 && p.1 >= 0 { points.push(p) }
            p = (p.0 + dx, p.1 + dy);
        }
    }
    points
}

pub fn solve(input: &str) -> (usize, usize) {
    let sensor_beacons = input.lines().map(|l| parse_line(l)).collect_vec();
    let beacons = sensor_beacons.iter().map(|&(_, b)| b).collect::<HashSet<_>>();
    let sensors = sensor_beacons.iter().map(|&(s, _)| s).collect::<HashSet<_>>();

    let p1 = sensor_beacons.iter().fold(HashSet::new(), |mut cover, &(s, b)| {
        let d = manhattan(s, b) as isize;
        cover.extend((s.0 - d..=s.0 + d).map(|x| (x, 2000000 as isize)).filter(|p| {
            !sensors.contains(p) && !beacons.contains(p) && manhattan(s, *p) <= d as usize
        }));
        cover
    }).len();

    let (x, y) = sensor_beacons.iter().flat_map(|&(s, b)| perimeter(s, b)).find(|&p| {
        sensor_beacons.iter().all(|&(s, b)| {
            manhattan(s, p) > manhattan(s, b)
        })
    }).unwrap();
    let p2 = x * 4000000 + y;

    (p1, p2 as usize)
}