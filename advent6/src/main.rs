use log::{debug, info};
use eyre::Result;

fn main() -> Result<()> {
    advent_common::setup_logging()?;

    let input = std::fs::read("input.txt")?;

    let result1 = find_start(&input, 4, 0) + 4;
    info!("result1: {}", result1);

    let result2 = find_start(&input, 14, 0) + 14;
    info!("result2: {}", result2);

    Ok(())
}

fn find_start(elems: &[u8], size: usize, index: usize) -> usize {
    match elems.get(0..size) {
        None => todo!(),
        Some(ref slice) => {
            debug!("{} checking: {:?}", index, slice);
            if all_different(slice) {
                index
            } else if let [_, ref tail @ ..] = elems {
                find_start(tail, size, index + 1)
            } else {
                todo!("FIXME")
            }
        }
    }
}

fn all_different<T: PartialEq>(slice: &[T]) -> bool {
    !slice.iter().fold(false, |acc, elem| {
        acc | (slice.iter().filter(|&another| elem == another).count() >= 2)
    })
}

#[test]
fn test_different() {
    assert_eq!(all_different(&[1,2,3,4]), true);
    assert_eq!(all_different(&[1,2,3,4,4]), false);
}
