pub use eyre;
pub use tracing;

pub fn file() -> Result<String, std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1) {
        None => std::fs::read_to_string("input_test.txt"),
        Some(val) => std::fs::read_to_string(val),
    }
}

pub fn setup_logging() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .without_time()
        .with_line_number(true)
        .init();

    color_eyre::install()?;

    Ok(())
}
