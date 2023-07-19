use advent_common::*;
use eyre::Result;
use tracing::info;

fn main() -> Result<()> {
    setup_logging()?;

    info!("hllo");

    Ok(())
}
