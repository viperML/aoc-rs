use std::fs;

use eyre::Result;

pub fn file() -> Result<String, std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1) {
        None => fs::read_to_string("input_test.txt"),
        Some(val) => std::fs::read_to_string(val),
    }
}

pub fn setup_logging() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .without_time()
        .with_line_number(true)
        .init();

    color_eyre::install()?;

    Ok(())
}
