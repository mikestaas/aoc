use clap::Parser;
use std::{collections::VecDeque, io};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    decrypt: bool,
}

fn main() {
    let Args { decrypt } = Args::parse();
    let mut input: VecDeque<(i64, usize)> = io::stdin()
        .lines()
        .enumerate()
        .map(|(i, n)| {
            (
                n.unwrap().parse::<i64>().unwrap() * if decrypt { 811589153 } else { 1 },
                i,
            )
        })
        .collect();
    (0..if decrypt { 10 } else { 1 }).for_each(|_| {
        (0..input.len()).for_each(|i| {
            let j = input.iter().position(|&(_, o)| o == i).unwrap();
            if let Some((n, _)) = input.remove(j) {
                let len = input.len() as i64;
                let j = ((((len + j as i64 + n) % len) + len) % len) as usize;
                input.insert(j, (n, i));
            }
        });
    });
    let offset = input.iter().position(|&(n, _)| n == 0).unwrap();
    let result: i64 = (1..=3)
        .map(|i| {
            let i = (i as usize * 1000 + offset) % input.len();
            let (n, _) = input.get(i).unwrap();

            n
        })
        .sum();
    println!("{}", result);
}
