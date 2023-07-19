use advent_common::*;
use eyre::Result;
use tracing::info;

#[derive(Debug, Default)]
struct Tree<'neighbour> {
    height: u64,

    up: Vec<&'neighbour Tree<'neighbour>>,
    down: Vec<&'neighbour Tree<'neighbour>>,
    left: Vec<&'neighbour Tree<'neighbour>>,
    right: Vec<&'neighbour Tree<'neighbour>>,
}

fn main() -> Result<()> {
    advent_common::setup_logging()?;
    let input = advent_common::file()?;

    let size = input.lines().count() as i32;
    info!(?size);

    let trees = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|t| t.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    eyre::ensure!(size * size == trees.len() as _, "Size was incorrect");

    let mut visibles = 0;

    let res = trees.iter().enumerate().map(|(idx, t)| {
        if (1 - size) <= 0 || size - 1 >= size * size {
            return true;
        }
        let range = (1 - size as usize)..(size as usize - 1);

        match trees.get(range) {
            None => true,
            Some(range) => range.iter().all(|elem| elem < t),
        }
    }).collect::<Vec<_>>();

    info!(?trees);
    info!(?res);

    Ok(())
}
