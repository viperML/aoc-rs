use log::{debug, info};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    advent_common::setup_logging()?;
    let file = advent_common::file()?;
    // let file = std::fs::read_to_string("input.txt")?;

    let contained = file
        .lines()
        .map(|line| {
            debug!("\n>{}<", line);

            let elements = line
                .split(&[',', '-'][..])
                .map(|e| e.parse().unwrap())
                .collect::<Vec<u32>>();

            debug!("{:?}", elements);

            let (left, right) = if let [a, b, c, d] = *elements.as_slice() {
                ((a, b), (c, d))
            } else {
                panic!("Didn't find 4 elements")
            };

            let right_contained = right.1 <= left.1 && right.0 >= left.0;
            let left_contained = left.1 <= right.1 && left.0 >= right.0;

            let result = right_contained | left_contained;
            debug!("{}", result);
            result
        })
        .collect::<Vec<_>>();

    let part1_result: u64 = contained
        .iter()
        .fold(0, |acc, &c| acc + if c { 1 } else { 0 });

    info!("part1 result: {}", part1_result);

    Ok(())
}
