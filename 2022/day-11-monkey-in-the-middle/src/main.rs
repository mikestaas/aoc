use clap::Parser;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use std::{
    collections::{HashMap, VecDeque},
    io,
};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    worry_big: bool,
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug)]
enum Operand {
    Num(u64),
    Old,
}

#[derive(Debug)]
struct Monkey {
    _id: u32,
    items: VecDeque<u64>,
    inspections: u64,
    operation: Operation,
    operand: Operand,
    divisor: u64,
    mt: usize,
    mf: usize,
}

fn monkey(input: &str) -> IResult<&str, Vec<Monkey>> {
    map(
        separated_list1(
            tag("\n\n"),
            tuple((
                preceded(tag("Monkey "), digit1),
                preceded(
                    tag(":\n  Starting items: "),
                    separated_list1(tag(", "), digit1),
                ),
                preceded(
                    tag("\n  Operation: new = old "),
                    separated_pair(anychar, tag(" "), alt((digit1, tag("old")))),
                ),
                preceded(tag("\n  Test: divisible by "), digit1),
                preceded(tag("\n    If true: throw to monkey "), digit1),
                preceded(tag("\n    If false: throw to monkey "), digit1),
            )),
        ),
        |monkeys| {
            monkeys
                .iter()
                .map(
                    |(id, items, (operation, operand), divisor, mt, mf): &(
                        &str,
                        Vec<&str>,
                        (char, &str),
                        &str,
                        &str,
                        &str,
                    )| Monkey {
                        _id: id.parse().unwrap(),
                        items: items.iter().map(|&i| i.parse().unwrap()).collect(),
                        inspections: 0,
                        operation: match operation {
                            '+' => Operation::Add,
                            '*' => Operation::Mul,
                            _ => panic!("unknown operation: {}", operation),
                        },
                        operand: match *operand {
                            "old" => Operand::Old,
                            num => Operand::Num(num.parse().unwrap()),
                        },
                        divisor: divisor.parse().unwrap(),
                        mt: mt.parse().unwrap(),
                        mf: mf.parse().unwrap(),
                    },
                )
                .collect()
        },
    )(input)
}

fn main() {
    let Args { worry_big } = Args::parse();
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .join("\n");
    let mut monkeys = monkey(&input).map(|(_, monkeys)| monkeys).unwrap();
    let mut next_items: HashMap<usize, VecDeque<u64>> = HashMap::new();
    let num_rounds = if worry_big { 10000 } else { 20 };
    let common_divisor: u64 = monkeys
        .iter()
        .map(|Monkey { divisor, .. }| *divisor)
        .product();

    (0..num_rounds).for_each(|_round| {
        (0..monkeys.len()).for_each(|i| {
            if let Some(Monkey {
                items,
                operation,
                operand,
                divisor,
                mf,
                mt,
                inspections,
                ..
            }) = monkeys.get_mut(i)
            {
                while let Some(item) = items.pop_front() {
                    let mut worry = match (&operation, &operand) {
                        (Operation::Add, Operand::Num(num)) => item + num,
                        (Operation::Add, Operand::Old) => item + item,
                        (Operation::Mul, Operand::Num(num)) => item * num,
                        (Operation::Mul, Operand::Old) => item * item,
                    };
                    if worry_big {
                        worry %= common_divisor;
                    } else {
                        worry /= 3;
                    }
                    let next = if worry % *divisor == 0 { *mt } else { *mf };
                    next_items
                        .entry(next)
                        .or_insert_with(VecDeque::new)
                        .push_back(worry);
                    *inspections += 1;
                }
            }
            next_items.iter_mut().for_each(|(&i, thrown)| {
                if let Some(Monkey { items, .. }) = monkeys.get_mut(i) {
                    items.append(thrown);
                }
            });
        });
    });

    let mut inspections: Vec<u64> = monkeys
        .iter()
        .map(|Monkey { inspections, .. }| *inspections)
        .collect();
    inspections.sort_unstable();
    let result: u64 = inspections.iter().rev().take(2).product();
    println!("{}", result);
}
