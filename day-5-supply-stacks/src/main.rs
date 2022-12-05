use clap::Parser;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::{cell::RefCell, collections::BTreeMap, io};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    multi: bool,
}

#[derive(Debug)]
struct Rearrangement {
    quantity: usize,
    from: usize,
    to: usize,
}

fn crate_layer(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(
        tag(" "),
        alt((
            preceded(tag("   "), tag("")),
            delimited(tag("["), take_until("]"), tag("]")),
        )),
    )(input)
}

fn num_preceded_by(pre: &str) -> impl Fn(&str) -> IResult<&str, usize> + '_ {
    move |i| map_res(preceded(tag(pre), digit1), str::parse::<usize>)(i)
}

fn procedure(input: &str) -> IResult<&str, Rearrangement> {
    map(
        tuple((
            num_preceded_by("move "),
            num_preceded_by(" from "),
            num_preceded_by(" to "),
        )),
        |(quantity, from, to)| Rearrangement { quantity, from, to },
    )(input)
}

fn main() {
    let Args { multi } = Args::parse();
    let input: Vec<String> = io::stdin().lines().map(Result::unwrap).collect();
    let mut supplies: BTreeMap<usize, RefCell<Vec<&str>>> = BTreeMap::new();
    let mut rearrangements: Vec<Rearrangement> = Vec::new();

    input.iter().for_each(|line| {
        if let Ok((_, layer)) = crate_layer(line) {
            layer.iter().enumerate().for_each(|(i, &supply)| {
                if !supply.is_empty() {
                    supplies
                        .entry(i + 1)
                        .or_insert(RefCell::new(Vec::new()))
                        .borrow_mut()
                        .push(supply);
                }
            });
        }

        if let Ok((_, rearrangement)) = procedure(line) {
            rearrangements.push(rearrangement);
        }
    });

    supplies
        .iter()
        .for_each(|(_, stack)| stack.borrow_mut().reverse());

    rearrangements
        .iter()
        .for_each(|&Rearrangement { quantity, from, to }| {
            let mut from = supplies.get(&from).unwrap().borrow_mut();
            let mut to = supplies.get(&to).unwrap().borrow_mut();
            if multi {
                let len = from.len();
                to.extend(from.split_off(len - quantity));
            } else {
                for _ in 0..quantity {
                    to.push(from.pop().unwrap());
                }
            }
        });

    let result = supplies
        .iter()
        .map(|(_, stack)| *stack.borrow().last().unwrap())
        .collect::<String>();

    println!("{}", result);
}
