use clap::Parser;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    multi::{many1, separated_list0},
    sequence::{delimited, separated_pair},
    IResult,
};
use std::{cmp::Ordering, io, str::FromStr};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    decode: bool,
}

#[derive(PartialEq, Clone, Debug)]
enum List<T> {
    List(Vec<List<T>>),
    Atom(T),
}

fn wrap<T: Clone>(l: &List<T>) -> List<T> {
    List::List(vec![l.clone()])
}

fn find<T: PartialEq>(l: &[List<T>], e: &List<T>) -> usize {
    l.iter().position(|i| i == e).map_or(0, |i| i + 1)
}

fn cmp<T: Clone + Ord>(left: &List<T>, right: &List<T>) -> Ordering {
    match (left, right) {
        (List::List(left), List::List(right)) => left
            .iter()
            .zip(right.iter())
            .find_map(|(left, right)| {
                let ord = cmp(left, right);
                if ord != Ordering::Equal {
                    Some(ord)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| left.len().cmp(&right.len())),
        (List::List(_), List::Atom(_)) => cmp(left, &wrap(right)),
        (List::Atom(_), List::List(_)) => cmp(&wrap(left), right),
        (List::Atom(left), List::Atom(right)) => left.cmp(right),
    }
}

fn list<T: FromStr>(input: &str) -> IResult<&str, List<T>> {
    map(
        delimited(
            tag("["),
            separated_list0(
                tag(","),
                alt((
                    list,
                    map_res(digit1, |n: &str| n.parse::<T>().map(List::Atom)),
                )),
            ),
            tag("]"),
        ),
        List::List,
    )(input)
}

type ListPair<T> = (List<T>, List<T>);

fn pairs<T: FromStr>(input: &str) -> IResult<&str, Vec<ListPair<T>>> {
    separated_list0(
        many1(line_ending),
        separated_pair(list, line_ending, list),
    )(input)
}

fn all<T: FromStr>(input: &str) -> IResult<&str, Vec<List<T>>> {
    separated_list0(many1(line_ending), list)(input)
}

fn main() {
    let Args { decode } = Args::parse();
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .join("\n");

    let result: usize = if decode {
        all::<i32>(&input).map_or(0, |(_, mut all)| {
            let two = wrap(&wrap(&List::Atom(2)));
            let six = wrap(&wrap(&List::Atom(6)));
            all.push(two.clone());
            all.push(six.clone());
            all.sort_by(cmp);

            find(&all, &two) * find(&all, &six)
        })
    } else {
        pairs::<i32>(&input).map_or(0, |(_, pairs)| {
            pairs
                .iter()
                .enumerate()
                .filter_map(|(i, (left, right))| {
                    if cmp(left, right) == Ordering::Less {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .sum()
        })
    };

    println!("{}", result);
}
