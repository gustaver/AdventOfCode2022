use std::iter::zip;
use itertools::Itertools;

fn to_digit(x: char) -> isize {
    match x {
        '0' | '1' | '2' => x.to_string().parse::<isize>().unwrap(),
        '-' => -1,
        '=' => -2,
        _ => unreachable!()
    }
}

fn to_snafu(x: isize) -> char {
    match x {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => unreachable!()
    }
}

fn add_digits(x: char, y:char, carry: isize) -> (isize, char) {
    let z = carry + to_digit(x) + to_digit(y);
    if z > 2 { (1, to_snafu(z - 5)) } else if z < -2 { (-1, to_snafu(z + 5)) } else { (0, to_snafu(z)) }
}

fn add(x: & str, y: &str) -> String {
    let n = x.len().max(y.len()) + 1;
    let x_pad = format!("{x:0>width$}", width=n);
    let y_pad = format!("{y:0>width$}", width=n);
    let z = zip(x_pad.chars().rev(), y_pad.chars().rev()).fold((0, "".to_string()), |(c, s), (a, b)| {
        let (carry, sum) = add_digits(a, b, c);
        (carry, format!("{sum}{s}"))
    }).1;
    if z.starts_with("0") { z[1..].to_string() } else { z }
}

pub fn solve(input: &str) -> (String, &str) {
    let numbers = input.lines().collect_vec();

    let p1 = numbers.iter().map(|&s| s.to_string()).reduce(|acc, n| add(&acc, &n)).unwrap();

    (p1, "https://giphy.com/gifs/reactionseditor-3oKIPf3C7HqqYBVcCk")
}