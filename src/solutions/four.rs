type Range = (usize, usize);

fn parse_range(assignmnet: &str) -> Range {
    if let [start, end] = assignmnet.split("-").collect::<Vec<_>>()[..] {
        return (start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap());
    }
    unreachable!();
}

fn parse_assigment(line: &str) -> (Range, Range) {
    if let [a1, a2] = line.split(",").collect::<Vec<_>>()[..] {
        return (parse_range(a1), parse_range(a2));
    }
    unreachable!();
}

pub fn solve(input: &str) -> (usize, usize) {
    let assignments = input.lines().map(parse_assigment).collect::<Vec<_>>();

    let p1: usize = assignments
        .iter()
        .map(|&((r1_s, r1_e), (r2_s, r2_e))| ((r1_s >= r2_s && r1_e <= r2_e) || (r2_s >= r1_s && r2_e <= r1_e)) as usize)
        .sum();

    let p2: usize = assignments
        .iter()
        .map(|&((r1_s, r1_e), (r2_s, r2_e))| (r1_e >= r2_s && r1_s <= r2_e) as usize)
        .sum();

    (p1, p2)
}