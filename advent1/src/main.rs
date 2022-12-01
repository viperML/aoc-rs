#![feature(str_split_whitespace_as_str)]

const FILENAME: &str = "input.txt";

fn main() -> anyhow::Result<()> {
    let file = std::fs::read_to_string(FILENAME)?;
    let elfs: Vec<Vec<i32>> = file
        .split("\n\n")
        .map(|lines| {
            lines
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    let elfs_total: Vec<_> = elfs
        .iter()
        .map(|e| e.iter().fold(0, |acc, x| acc + x))
        .collect();

    let result = elfs_total
        .iter()
        .fold(0, |acc, &x| if acc > x { acc } else { x });

    println!("{:?}", result);

    Ok(())
}
