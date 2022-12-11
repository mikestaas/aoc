use clap::Parser;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, rest},
    sequence::{pair, separated_pair},
    IResult,
};
use std::io;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    draw: bool,
}

#[derive(Debug)]
enum Operation {
    AddX(i32),
    NoOp,
}

fn operation(input: &str) -> IResult<&str, Operation> {
    map(
        alt((
            pair(tag("noop"), tag("")),
            separated_pair(tag("addx"), tag(" "), rest),
        )),
        |(operation, argument)| match operation {
            "noop" => Operation::NoOp,
            "addx" => Operation::AddX(argument.parse().unwrap()),
            _ => panic!("invalid operation: {}", operation),
        },
    )(input)
}

fn main() {
    let Args { draw } = Args::parse();
    let input: Vec<Operation> = io::stdin()
        .lines()
        .map(|line| {
            operation(&line.unwrap())
                .map(|(_, operation)| operation)
                .unwrap()
        })
        .collect();
    let mut x = 1;
    let mut values = vec![];

    input.iter().for_each(|op| match op {
        Operation::AddX(num) => {
            values.push(x);
            values.push(x);
            x += num;
        }
        Operation::NoOp => values.push(x),
    });

    let result = if draw {
        values
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                if (x - 1..=x + 1).contains(&(i as i32 % 40)) {
                    '#'
                } else {
                    '.'
                }
            })
            .collect::<Vec<char>>()
            .chunks(40)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        values
            .iter()
            .enumerate()
            .skip(19)
            .step_by(40)
            .map(|(i, &x)| (i as i32 + 1) * x)
            .sum::<i32>()
            .to_string()
    };

    println!("{}", result);
}
