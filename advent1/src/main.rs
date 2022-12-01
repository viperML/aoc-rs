#![feature(str_split_whitespace_as_str)]

use std::{error::Error, result};

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string(FILENAME)?;
    let elfs: Vec<Vec<i32>> = file
        .split("\n\n")
        .map(|lines| {
            lines
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    let mut elfs_total: Vec<_> = elfs
        .iter()
        .map(|e| e.iter().fold(0, |acc, x| acc + x))
        .collect();

    elfs_total.sort_by(|a, b| b.cmp(a));

    if let [first, second, third, _tail @ ..] = &*elfs_total {
        println!("result1: {}", first);
        println!("result2: {}", first + second + third);
    }

    Ok(())
}
