use log::{debug, info};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    advent_common::setup_logging()?;
    let file = advent_common::file()?;
    // let file = std::fs::read_to_string("input.txt")?;

    let pairs: Vec<_> = file
        .lines()
        .map(|line| {
            let elements = line
                .split(&[',', '-'][..])
                .map(|e| e.parse().unwrap())
                .collect::<Vec<u32>>();

            if let [a, b, c, d] = *elements.as_slice() {
                ((a, b), (c, d))
            } else {
                panic!("Didn't find 4 elements")
            }
        })
        .collect();

    let part1_result = pairs
        .iter()
        .map(|(left, right)| {
            let right_contained = right.1 <= left.1 && right.0 >= left.0;
            let left_contained = left.1 <= right.1 && left.0 >= right.0;
            right_contained | left_contained
        })
        .fold(0, |acc, c| acc + if c { 1 } else { 0 });

    let part2_result = pairs
        .iter()
        .map(|(left, right)| {
            let left_size = left.1 - left.0;
            let right_size = right.1 - right.0;

            let total_size = (if right.1 >= left.1 { right.1 } else { left.1 })
                - (if right.0 <= left.0 { right.0 } else { left.0 });

            total_size <= left_size + right_size
        })
        .fold(0, |acc, c| acc + if c { 1 } else { 0 });

    info!("part1 result: {}", part1_result);
    info!("part1 result: {}", part2_result);

    Ok(())
}
