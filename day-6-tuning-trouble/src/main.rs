use clap::Parser;
use std::{collections::HashSet, io};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    message: bool,
}

fn find_first_unique(input: &str, length: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(length)
        .position(|w| HashSet::<_>::from_iter(w).len() == length)
        .unwrap()
        + length
}

fn main() {
    let Args { message } = Args::parse();
    let input = io::stdin().lines().next().unwrap().unwrap();
    let length = if message { 14 } else { 4 };
    let offset = find_first_unique(&input, length);

    println!("{:?}", offset);
}
