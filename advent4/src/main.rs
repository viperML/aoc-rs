use std::error::Error;
use log::debug;

fn main() -> Result<(), Box<dyn Error>> {
    advent_common::setup_logging()?;
    let file = advent_common::file()?;

    debug!("{}", file);

    Ok(())
}
