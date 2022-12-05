use clap::Parser;
use itertools::Itertools;
use std::{io, ops::RangeInclusive};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    overlap: bool,
}

trait Compare<RangeInclusive> {
    fn contains(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl<T: std::cmp::PartialOrd> Compare<RangeInclusive<T>> for RangeInclusive<T> {
    fn contains(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.start() <= other.end() && self.end() >= other.start()
    }
}

fn main() {
    let Args { overlap } = Args::parse();
    let input: Vec<String> = io::stdin().lines().map(Result::unwrap).collect();
    let overlaps = input.iter().fold(0, |mut overlaps, line| {
        let (first, second) = line
            .split(',')
            .map(|range| {
                range
                    .split('-')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .map(|(start, end)| RangeInclusive::new(start, end))
            .collect_tuple()
            .unwrap();
        if overlap && first.overlaps(&second)
            || Compare::contains(&first, &second)
            || Compare::contains(&second, &first)
        {
            overlaps += 1;
        }
        overlaps
    });
    println!("{}", overlaps);
}
