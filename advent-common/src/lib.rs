use eyre::Result;
use fern::colors::Color;

pub fn file() -> Result<String, std::io::Error> {
    let filename = if cfg!(debug_assertions) {
        "input_test.txt"
    } else {
        "input.txt"
    };

    std::fs::read_to_string(filename)
}

pub fn setup_logging() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    color_eyre::install()?;

    Ok(())
}
