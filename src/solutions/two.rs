// P1:
// X (Rock) -> 1, Y (Paper) -> 2, Z (Scissors) -> 3
// A (Rock), B (Paper), C (Scissors)

// P2:
// X -> lose, Y -> draw, Z -> win

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

#[derive(Copy, Clone)]
enum Outcome {
    Lose,
    Draw,
    Win
}

fn round_outcome(opponent: Shape, you: Shape) -> i32 {
    match you {
        Shape::Rock => match opponent {
            Shape::Rock => 3,
            Shape::Paper => 0,
            Shape::Scissors => 6
        },
        Shape::Paper => match opponent {
            Shape::Rock => 6,
            Shape::Paper => 3,
            Shape::Scissors => 0
        },
        Shape::Scissors => match opponent {
            Shape::Rock => 0,
            Shape::Paper => 6,
            Shape::Scissors => 3
        }
    }
}

fn outcome_to_shape(opponent: Shape, outcome: Outcome) -> Shape {
    match outcome {
        Outcome::Lose => match opponent {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper
        },
        Outcome::Draw => opponent,
        Outcome::Win => match opponent {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock
        }
    }
}

fn parse_round(round: &str) -> (&str, &str) {
    if let [opponent, you] = round.split_whitespace().collect::<Vec<_>>()[..] {
        return (opponent, you);
    }
    panic!("Wasn't able to destructure!");
}

fn to_shape(play: &str) -> Shape {
    match play {
        "X" | "A" => Shape::Rock,
        "Y" | "B" => Shape::Paper,
        "Z" | "C" => Shape::Scissors,
        _ => panic!("Got invalid shape in to_shape")
    }
}

fn to_outcome(o: &str) -> Outcome {
    match o {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Got invalid outcome in to_outcome")
    }
}

fn shape_to_score(shape: Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3
    }
}

fn round_score(round: &str) -> i32 {
    let (opponent, you) = parse_round(round);
    let (opponent, you) = (to_shape(opponent), to_shape(you));
    let shape_score = shape_to_score(you.clone());
    let outcome_score = round_outcome(opponent, you);

    shape_score + outcome_score
}

fn choose_shape(round: &str) -> i32 {
    let (opponent, outcome) = parse_round(round);
    let (opponent, outcome) = (to_shape(opponent), to_outcome(outcome));
    let you = outcome_to_shape(opponent, outcome);
    let shape_score = shape_to_score(you.clone());
    let outcome_score = round_outcome(opponent, you);

    shape_score + outcome_score
}

pub fn solve(input: &str) -> (i32, i32) {
    let rounds = input.lines().collect::<Vec<_>>();
    
    let p1: i32 = rounds.iter().map(|r| round_score(r)).sum();
    let p2: i32 = rounds.iter().map(|r| choose_shape(r)).sum();

    (p1, p2)
}