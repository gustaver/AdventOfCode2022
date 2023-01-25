use itertools::Itertools;
use regex::Regex;
use lazy_static::lazy_static;

fn time_to_robot(robots: [usize; 4], resources: [usize; 4], cost: [usize; 4], time: usize, r: usize) -> (usize, [usize; 4], [usize; 4]) {
    let mut res = resources;
    let mut rob = robots;
    for t in 1usize..=time {
        if res.iter().enumerate().all(|(i, &r)| r >= cost[i]) {
            for i in 0..res.len() { res[i] += robots[i] }
            for i in 0..res.len() { res[i] -= cost[i] }
            rob[r] += 1;
            return (time - t, res, rob)
        }
        for i in 0..res.len() { res[i] += robots[i] }
    }
    (0, res, rob)
}

fn max_geodes(blueprint: [[usize; 4]; 4], max_robots: [usize; 4], robots: [usize; 4], resources: [usize; 4], time: usize) -> usize {
    if time == 0 { return resources[3] }
    (0..4usize).filter(|&robot| (robot == 3 || robots[robot] < max_robots[robot])).map(|robot| {
        let cost = blueprint[robot];
        let (t, res, rob) = time_to_robot(robots, resources, cost, time, robot);
        max_geodes(blueprint, max_robots, rob, res, t)
    }).max().unwrap()
}

fn parse_line(line: &str) -> [[usize; 4]; 4] {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    }
    let captures = RE.captures(line).unwrap();
    // robots: [ore, clay, obsidian, geode]
    // cost: [ore, clay, obsidian, geode]
    [
        [captures.get(1).unwrap().as_str().parse().unwrap(), 0, 0, 0],
        [captures.get(2).unwrap().as_str().parse().unwrap(), 0, 0, 0],
        [captures.get(3).unwrap().as_str().parse().unwrap(), captures.get(4).unwrap().as_str().parse().unwrap(), 0, 0],
        [captures.get(5).unwrap().as_str().parse().unwrap(), 0, captures.get(6).unwrap().as_str().parse().unwrap(), 0],
    ]
}

pub fn solve(input: &str) -> (usize, usize) {
    let blueprints = input.lines().map(parse_line).collect_vec();

    let p1 = blueprints.iter().enumerate().map(|(i, &bp)| {
        let mut max_robots = [0, 0, 0, 0];
        for rob in bp {
            for (r, &c) in rob.iter().enumerate() {
                max_robots[r] = c.max(max_robots[r]);
            } 
        }
        max_geodes(bp, max_robots, [1, 0, 0, 0], [0, 0, 0, 0], 24) * (i + 1)
    }).sum();
    let p2 = blueprints.iter().take(3).map(|&bp| {
        let mut max_robots = [0, 0, 0, 0];
        for rob in bp {
            for (r, &c) in rob.iter().enumerate() {
                max_robots[r] = c.max(max_robots[r]);
            } 
        }
        max_geodes(bp, max_robots, [1, 0, 0, 0], [0, 0, 0, 0], 32)
    }).product();

    (p1, p2)
}