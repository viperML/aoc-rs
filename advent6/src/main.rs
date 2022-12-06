use log::{debug, info};

fn main() -> anyhow::Result<()> {
    advent_common::setup_logging()?;

    let input = std::fs::read("input.txt")?;
    debug!("{:?}", input);

    // input.iter().fold(None, |acc: Option<usize>, x| todo!());
    let result1 = find_start(&input, 0) + 4;
    info!("result1: {}", result1);

    Ok(())
}

fn find_start(elems: &[u8], index: usize) -> usize {
    match elems {
        [a, b, c, y, ..] => {
            let slice = &[a, b, c, y];

            let all_different = !slice.iter().fold(false, |acc, x| {
                acc | (slice.iter().filter(|&y| x == y).count() >= 2)
            });
            // .fold(false, |result, x| result | slice.contains(x));

            debug!("{} Checking: {:?} -> {}", index, slice, all_different);

            if all_different {
                index
            } else if let [_, tail @ ..] = elems {
                find_start(tail, index + 1)
            } else {
                todo!("FIXME")
            }
        }
        _ => todo!("FIXME"),
    }
}
