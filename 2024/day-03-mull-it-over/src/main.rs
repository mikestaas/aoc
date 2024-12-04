use std::io;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{char, digit1},
    combinator::map,
    multi::{many0, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

enum Instuction {
    Do,
    Dont,
    Mul(u32, u32),
}

fn get_instructionsx(input: &str) -> IResult<&str, Vec<Instuction>> {
    many0(map(
        many_till(
            take(1usize),
            alt((
                map(tag("do()"), |_| Instuction::Do),
                map(tag("don't()"), |_| Instuction::Dont),
                map(
                    delimited(
                        tag("mul("),
                        separated_pair(digit1::<&str, _>, char(','), digit1),
                        char(')'),
                    ),
                    |(l, r)| Instuction::Mul(l.parse().unwrap(), r.parse().unwrap()),
                ),
            )),
        ),
        |(_, instr)| instr,
    ))(input)
}

fn main() {
    let args: Vec<Instuction> = io::stdin()
        .lines()
        .map(|line| {
            get_instructionsx(&line.unwrap())
                .map(|(_, instr)| instr)
                .unwrap()
        })
        .flatten()
        .collect();

    let result: u32 = args
        .iter()
        .map(|instr| match instr {
            Instuction::Do => 0,
            Instuction::Dont => 0,
            Instuction::Mul(l, r) => l * r,
        })
        .sum();

    let (conditional, _) = args
        .iter()
        .fold((0, true), |(mut sum, mut enabled), instr| {
            match instr {
                Instuction::Do => {
                    enabled = true;
                }
                Instuction::Dont => {
                    enabled = false;
                }
                Instuction::Mul(l, r) => {
                    if enabled {
                        sum += l * r;
                    }
                }
            };

            (sum, enabled)
        });

    println!("result: {}", result);
    println!("conditional: {}", conditional);
}
