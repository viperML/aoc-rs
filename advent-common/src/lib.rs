use fern::colors::Color;

pub fn file() -> Result<String, std::io::Error> {
    let filename = if cfg!(debug_assertions) {
        "input_test.txt"
    } else {
        "input.txt"
    };

    std::fs::read_to_string(filename)
}

pub fn setup_logging() -> Result<(), log::SetLoggerError> {
    let loglevel = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    let color_text = fern::colors::ColoredLevelConfig::new()
        .debug(Color::BrightBlack)
        .error(Color::White)
        .trace(Color::BrightBlue);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}{message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    color_text.get_color(&record.level()).to_fg_str()
                ),
                message = message,
            ));
        })
        .level(loglevel)
        .chain(std::io::stdout())
        .apply()
}
