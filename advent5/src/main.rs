use std::{collections::VecDeque, error::Error, result};

use anyhow::{anyhow, bail};
use log::{debug, info};

fn main() -> Result<(), Box<dyn Error>> {
    advent_common::setup_logging()?;
    // let file = std::fs::read_to_string("input.txt")?;
    let file = advent_common::file()?;

    let re_index = regex::Regex::new(r"(\d+)")?;

    let indices: Vec<Vec<u32>> = file
        .lines()
        .map(|line| {
            debug!(">{}<", line);
            let indeces_matches: Vec<_> = re_index
                .captures_iter(line)
                .filter_map(|cap| cap.get(0).and_then(|m| m.as_str().parse().ok()))
                .collect();
            debug!("{:?}", indeces_matches);
            indeces_matches
        })
        .filter(|indeces_mactches| !indeces_mactches.is_empty())
        .collect();

    let (columns, steps) = if let [head, tail @ ..] = indices.as_slice() {
        let _steps: Vec<_> = tail
            .iter()
            .map(|elems| (elems[0], elems[1], elems[2]))
            .collect();

        Ok((head[head.len() - 1], _steps))
    } else {
        Err(anyhow!("Indices was empty"))
    }?;

    debug!("Steps: {:?}", steps);
    info!("Columns: {}", columns);

    let re_blocks = regex::Regex::new(r"([A-Z])+")?;

    let blocks_regex: Vec<Vec<_>> = file
        .lines()
        .rev()
        .map(|line| {
            re_blocks
                .captures_iter(line)
                .filter_map(|cap| cap.get(0))
                .collect::<Vec<_>>()
        })
        .filter(|c| !c.is_empty())
        .collect();

    let mut blocks = vec![Vec::new(); columns as usize];

    for b in blocks_regex {
        for matching in b {
            let matching_column = (matching.end() + 2) / 4 - 1;
            blocks[matching_column].push(matching.as_str());
        }
    }

    info!("{:?}", blocks);

    let blocks_1 = cratemover9000(&blocks, &steps);
    info!("blocks1: {:?}", blocks_1);
    let result_1 = block_tops(blocks_1);
    info!("result1: {}", result_1);

    let blocks_2 = cratemover9001(&blocks, &steps);
    info!("blocks2: {:?}", blocks_2);
    let result_2 = block_tops(blocks_2);
    info!("result2: {}", result_2);

    Ok(())
}

fn cratemover9000<'a>(
    blocks: &Vec<Vec<&'a str>>,
    steps: &Vec<(u32, u32, u32)>,
) -> Vec<Vec<&'a str>> {
    let blocks = blocks.clone();

    let blocks = steps.iter().fold(blocks, |mut acc, step| {
        debug!("{:?}", step);
        for _ in 0..step.0 {
            debug!("Moving {}->{}", step.1, step.2);
            let x = acc[step.1 as usize - 1]
                .pop()
                .expect("Tried to move nonexisting block");
            acc[step.2 as usize - 1].push(x);
        }

        acc
    });

    blocks
}

fn cratemover9001<'a>(
    blocks: &Vec<Vec<&'a str>>,
    steps: &Vec<(u32, u32, u32)>,
) -> Vec<Vec<&'a str>> {
    let blocks = blocks.clone();

    let blocks = steps.iter().fold(blocks, |mut acc, step| {
        debug!("{:?}", step);
        let mut stack = Vec::new();
        for _ in 0..step.0 {
            let x = acc[step.1 as usize - 1]
                .pop()
                .expect("Tried to move nonexisting block");

            stack.push(x);
        }

        debug!("stack: {:?}", stack);
        stack.reverse();

        acc[step.2 as usize - 1].append(&mut stack);

        acc
    });

    blocks
}

fn block_tops(blocks: Vec<Vec<&str>>) -> String {
    blocks
        .iter()
        .fold("".to_string(), |acc, column| acc + column[column.len() - 1])
}
