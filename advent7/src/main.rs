use eyre::Result;
use tracing::info;

fn main() -> Result<()> {
    advent_common::setup_logging()?;
    let file = advent_common::file()?;
    // debug!("{}", file);

    let test = std::fs::canonicalize("/a/../c");
    info!(?test);


    Ok(())
}
