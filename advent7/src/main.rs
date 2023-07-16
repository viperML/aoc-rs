use std::str::Lines;

use eyre::Result;
use tracing::{info, span};
use tracing::{warn, Level};

const TOTAL: u64 = 70000000;
const UNUSED_REQUIRED: u64 = 30000000;

fn main() -> Result<()> {
    advent_common::setup_logging()?;
    let file = advent_common::file()?;

    let mut root_dir = Dir {
        name: String::from("/"),
        ..Default::default()
    };

    let mut lines = file.lines();
    lines.next(); // Skip first line

    let mut sum = 0;

    walk(&mut lines, &mut root_dir, &mut sum);

    info!("{root_dir:#?}");
    info!(?sum);

    let used = root_dir.size;
    let available = TOTAL - used;
    let to_delete = UNUSED_REQUIRED - available;

    info!(?used, ?available, ?to_delete);

    let mut deletion = root_dir.size;
    walk_delete(&root_dir, &mut deletion, to_delete);

    info!(?deletion);

    Ok(())
}

fn walk_delete(dir: &Dir, deletion: &mut u64, minimum: u64) {
    for sub_dir in dir.sub_dirs.iter() {
        walk_delete(sub_dir, deletion, minimum);
    }

    if dir.size >= minimum && dir.size <= *deletion {
        *deletion = dir.size;
    }
}

fn walk(lines: &mut Lines, dir: &mut Dir, sum: &mut u64) {
    let line = match lines.next() {
        None => return,
        Some(s) => s,
    };

    let token = parse_line(line);
    info!(?line, ?token);

    match token {
        Token::Ls => {}
        Token::Dir(_) => {}
        Token::ChangeDir(None) => {
            if dir.size <= 100_000 {
                *sum += dir.size;
            }
            dir.added = true;
            return;
        }
        Token::ChangeDir(Some(mut new_dir)) => {
            info!("Adding!");
            walk(lines, &mut new_dir, sum);
            dir.size += new_dir.size;
            dir.sub_dirs.push(new_dir);
        }
        Token::File(file) => {
            dir.size += file.size;
            dir.files.push(file);
        }
    }

    walk(lines, dir, sum);
}

#[derive(Debug, Default)]
struct File {
    name: String,
    size: u64,
}

#[derive(Debug, Default)]
struct Dir {
    name: String,
    sub_dirs: Vec<Dir>,
    files: Vec<File>,
    size: u64,
    added: bool,
}

#[derive(Debug)]
enum Token {
    ChangeDir(Option<Dir>),
    File(File),
    Dir(Dir),
    Ls,
}

fn parse_line(line: &str) -> Token {
    if line.starts_with("$ cd ..") {
        Token::ChangeDir(None)
    } else if line.starts_with("$ cd") {
        Token::ChangeDir(Some(Dir {
            name: line.split(" ").last().unwrap().to_string(),
            ..Default::default()
        }))
    } else if line == "$ ls" {
        Token::Ls
    } else if line.starts_with("dir") {
        Token::Dir(Dir {
            name: line.split(" ").last().unwrap().to_string(),
            ..Default::default()
        })
    } else {
        let mut components = line.split(" ");
        Token::File(File {
            size: components.next().unwrap().parse().unwrap(),
            name: components.next().unwrap().to_string(),
        })
    }
}
