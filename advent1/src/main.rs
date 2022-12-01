#![feature(str_split_whitespace_as_str)]

use std::error::Error;

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

    println!("result1: {:?}", elfs_total);

    let result1 = elfs_total
        .iter()
        .fold(0, |acc, &x| if acc > x { acc } else { x });

    println!("result1: {:?}", result1);

    let (first_three, _) = elfs_total.split_at(3);
    let result2 = first_three.iter().fold(0, |acc, x| acc + x);

    println!("result2: {:?}", result2);

    Ok(())
}
