use std::{collections::HashSet, error::Error, result};

use fern::colors::Color;
use log::debug;

const CONVERSION: &str = "------------ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcba";

fn main() -> Result<(), Box<dyn Error>> {
    setup_logging()?;
    let filename = if cfg!(debug_assertions) {
        "input_test.txt"
    } else {
        "input.txt"
    };
    let file = std::fs::read_to_string(filename)?;

    let rucksacks = file
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect::<Vec<_>>();

    let scores = rucksacks
        .iter()
        .map(|(left, right)| {
            debug!("{}<>{}", left, right);
            let common_letters_mask = str_to_mask(left) & str_to_mask(right);
            debug!("0b{}", CONVERSION);
            debug!("{:#066b}", common_letters_mask);

            let result = (0..52).fold(0, |acc, i| {
                let mask = 1 << i;
                let value = common_letters_mask & mask;
                acc + if value != 0 { i + 1 } else { 0 }
            });
            debug!("{}", result);
            result
        })
        .collect::<Vec<u64>>();

    let result: u64 = scores.iter().sum();
    println!("result: {}", result);

    Ok(())
}

fn str_to_mask(s: &str) -> u64 {
    let bytes = s
        .bytes()
        .map(|b| if b >= 97 { b - 97 } else { b - 39 })
        .collect::<HashSet<_>>();

    let mut result = 0;

    // println!("{:?}", bytes);

    for b in &bytes {
        let x: u64 = 1 << b;
        result = result | &x;
        // println!("{:#066b}", x);
    }

    result
}

fn setup_logging() -> Result<(), log::SetLoggerError> {
    let loglevel = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    let color_text = fern::colors::ColoredLevelConfig::new()
        .debug(Color::BrightBlack)
        .error(Color::White)
        .trace(Color::BrightBlue);

    let color_symbol = fern::colors::ColoredLevelConfig::new()
        .debug(Color::BrightBlack)
        .error(Color::Red)
        .error(Color::Red)
        .info(Color::Green)
        .trace(Color::BrightBlue)
        .warn(Color::Yellow);

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
