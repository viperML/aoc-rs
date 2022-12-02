use itertools::Itertools;
use std::error::Error;

const FILENAME: &str = "input.txt";

#[derive(Debug, PartialEq, Eq)]
enum Shape {
    // Rock
    A,
    // Paper
    B,
    // Scissors
    C,
}

fn get_shape(input: &str) -> Option<Shape> {
    match input {
        "X" | "A" => Some(Shape::A),
        "Y" | "B" => Some(Shape::B),
        "Z" | "C" => Some(Shape::C),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string(FILENAME)?;

    let rounds = file
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .collect_tuple()
                .and_then(|(a, b)| {
                    if let (Some(x), Some(y)) = (get_shape(a), get_shape(b)) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
        })
        .collect::<Option<Vec<(_, _)>>>()
        .expect("Couldn't parse input!");

    let scores_problem_1 = rounds.iter().map(|r| compute_score_1(r)).collect::<Vec<_>>();
    let score_1: i32 = scores_problem_1.iter().sum();
    println!("{}", score_1);

    let scores_problem_2 = rounds.iter().map(|r| compute_score_2(r)).collect::<Vec<_>>();
    let score_2: i32 = scores_problem_2.iter().sum();
    println!("{}", score_2);

    Ok(())
}

fn compute_score_1(round: &(Shape, Shape)) -> i32 {
    let match_score = if round.0 == round.1 {
        3
    } else if let (Shape::C, Shape::A) | (Shape::B, Shape::C) | (Shape::A, Shape::B) = round {
        6
    } else {
        0
    };

    compute_shape_score(&round.1) + match_score
}

fn compute_shape_score(shape: &Shape) -> i32 {
    match shape {
        Shape::A => 1,
        Shape::B => 2,
        Shape::C => 3,
    }
}

fn compute_score_2(round: &(Shape, Shape)) -> i32 {
    /*
        round.1
        Shape::A => Lose
        Shape::B => Tie
        Shape::C => Win
    */
    let match_score = match round.1 {
        Shape::A => 0,
        Shape::B => 3,
        Shape::C => 6,
    };

    let shape_score = if round.1 == Shape::B {
        compute_shape_score(&round.0)
    } else if round.1 == Shape::C {
        compute_shape_score(&match round.0 {
            Shape::A => Shape::B,
            Shape::B => Shape::C,
            Shape::C => Shape::A,
        })
    } else {
        compute_shape_score(&match round.0 {
            Shape::A => Shape::C,
            Shape::B => Shape::A,
            Shape::C => Shape::B,
        })
    };

    shape_score + match_score
}
