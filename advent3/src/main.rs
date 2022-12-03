use std::error::Error;

const FILENAME: &str = "input_test.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string(FILENAME)?;

    println!(
        "{:?}",
        "abcxyz".bytes().map(|b| value(b)).collect::<Vec<_>>()
    );
    println!(
        "{:?}",
        "ABCXYZ".bytes().map(|b| value(b)).collect::<Vec<_>>()
    );

    let rucksacks = file
        .lines()
        .map(|line| line.as_bytes().split_at(line.len() / 2))
        .collect::<Vec<_>>();

    let scores: Vec<_> = rucksacks
        .iter()
        .map(|(letters_left, letters_right)| {
            // letters_left.intersect(letters_right)
            println!("checking {:?}<>{:?}", letters_left, letters_right);
            letters_left.iter().fold(0, |acc, &l_left| {
                acc + {
                    if letters_right.contains(&l_left) {
                        println!("contains {}", l_left);
                        value(l_left)
                    } else {
                        0
                    }
                }
            })
        })
        .collect();

    println!("{:?}", rucksacks);
    println!("{:?}", scores);

    // if let [head, tail @ ..] = &*rucksacks {
    //     let letters_left = head.0.as_bytes();
    //     let letter_right = head.1.as_bytes();

    //     let z: u64 = letters_left.iter().fold(0, |acc, &letter_left| {
    //         letter_right.iter().fold(0, |acc_y, &letter_right| {
    //             if letter_left == letter_right {
    //                 acc + value(letter_left)
    //             } else {
    //                 acc
    //             }
    //         })
    //     });

    //     println!("{:?}", letters_left);
    //     println!("{:?}", letter_right);
    //     println!("{:?}", z);
    // }

    Ok(())
}

fn value(letter: u8) -> u64 {
    if letter <= 90 {
        letter - 64 + 26
    } else {
        letter - 96
    }
    .into()
}
