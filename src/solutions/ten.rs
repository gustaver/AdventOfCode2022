use itertools::Itertools;

pub fn solve(input: &str) -> (isize, String) {
    let instructions = input.lines().collect::<Vec<_>>();

    
    let x_register = instructions.iter().fold(vec![1], |mut hist, &inst| {
        let x = hist[hist.len() - 1];
        hist.push(x);
        match inst {
            addx if inst.starts_with("addx") => {
                let (_, y) = addx.split_once(" ").unwrap();
                let y = y.parse::<isize>().unwrap();
                hist.push(x + y);
            },
            _ => ()
        };
        hist
    });

    let p1 = [20, 60, 100, 140, 180, 220].map(|cycle| x_register[cycle - 1] * (cycle as isize)).iter().sum();

    let crt = x_register.iter().enumerate().map(|(pixel, &x)| {
        let pixel = pixel % 40;
        if x.abs_diff(pixel as isize) < 2 {
            '#'
        } else {
            '.'
        }
    }).collect::<Vec<_>>();

    let p2 = crt.chunks(40).map(|row| row.iter().collect::<String>()).join("\n");
    (p1, format!("\n{}", p2))
}