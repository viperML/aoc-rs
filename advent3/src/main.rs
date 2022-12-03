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
    let lines = file.lines().collect::<Vec<_>>();

    let rucksacks = lines
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .collect::<Vec<_>>();

    let part1_scores = rucksacks
        .iter()
        .map(|(left, right)| {
            debug!("{}<>{}", left, right);
            let common_letters_mask = str_to_mask(left) & str_to_mask(right);
            debug!("0b{}", CONVERSION);
            debug!("{:#066b}", common_letters_mask);

            let result = mask_score(common_letters_mask);
            debug!("{}", result);
            result
        })
        .collect::<Vec<u64>>();

    let part1_result: u64 = part1_scores.iter().sum();
    println!("part 1 result: {}", part1_result);

    let groups = lines.chunks(3).collect::<Vec<_>>();

    let part2_scores = groups
        .iter()
        .map(|&elem| {
            if let [first, second, third] = elem {
                debug!("{}\n{}\n{}", first, second, third);
                let common_letters_mask =
                    str_to_mask(first) & str_to_mask(second) & str_to_mask(third);
                debug!("0b{}", CONVERSION);
                debug!("{:#066b}", common_letters_mask);
                let score = mask_score(common_letters_mask);
                score
            } else {
                panic!("Failed to split in groups of 3: {:?}", elem);
            }
        })
        .collect::<Vec<_>>();

    let part2_result: u64 = part2_scores.iter().sum();
    println!("part 2 result: {}", part2_result);

    Ok(())
}

fn str_to_mask(s: &str) -> u64 {
    let bytes = s
        .bytes()
        .map(|b| if b >= 97 { b - 97 } else { b - 39 })
        .collect::<HashSet<_>>();

    let mut result = 0;

    for b in &bytes {
        result = result | &(1 << b);
    }

    result
}

fn mask_score(mask: u64) -> u64 {
    (0..52).fold(0, |acc, i| {
        let filter_mask = 1 << i;
        let value = mask & filter_mask;
        acc + if value != 0 { i + 1 } else { 0 }
    })
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
