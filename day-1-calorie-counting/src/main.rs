use clap::Parser;
use std::io;

#[derive(Parser)]
struct Args {
    #[arg(default_value_t = 1)]
    num: usize,
}

fn main() {
    let Args { num } = Args::parse();
    let input: Vec<String> = io::stdin().lines().map(Result::unwrap).collect();
    let mut calories = input.iter().fold(vec![0], |mut calories, line| {
        if line.is_empty() {
            calories.push(0);
        }
        if let (Some(total), Ok(amount)) = (calories.last_mut(), line.parse::<u32>()) {
            *total += amount
        }
        calories
    });
    calories.sort_unstable();
    println!("{}", calories.iter().rev().take(num).sum::<u32>());
}
