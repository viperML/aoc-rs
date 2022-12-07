use log::debug;

fn main() -> anyhow::Result<()> {
    advent_common::setup_logging()?;
    let file = advent_common::file()?;
    // debug!("{}", file);

    let test = std::fs::canonicalize("/a/../c");
    debug!("{:?}", test);


    Ok(())
}
