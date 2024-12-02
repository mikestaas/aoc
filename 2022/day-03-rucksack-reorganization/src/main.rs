use clap::Parser;
use std::io;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    badges: bool,
}

fn common<T: AsRef<str>>(strings: &[T]) -> Vec<char> {
    strings
        .iter()
        .enumerate()
        .fold(vec![], |common, (i, string)| {
            if i == 0 {
                string.as_ref().chars().collect()
            } else {
                common
                    .into_iter()
                    .filter(|char| string.as_ref().contains(*char))
                    .collect()
            }
        })
}

fn priority(char: char) -> u32 {
    let ord = char as u32;
    if char.is_ascii_lowercase() {
        ord - 96
    } else if char.is_ascii_uppercase() {
        ord - 38
    } else {
        0
    }
}

fn main() {
    let Args { badges } = Args::parse();
    let input: Vec<String> = io::stdin().lines().map(Result::unwrap).collect();
    let sum = if badges {
        input
            .chunks(3)
            .map(common)
            .map(|chars| priority(*chars.first().unwrap()))
            .sum()
    } else {
        input.iter().fold(0, |mut sum, line| {
            let (first, second) = line.split_at(line.len() / 2);
            if let Some(&item) = common(&[first, second]).first() {
                sum += priority(item);
            }
            sum
        })
    };
    println!("{}", sum);
}
