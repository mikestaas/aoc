use clap::Parser;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::{preceded, tuple},
    IResult,
};
use std::{
    collections::{HashSet, VecDeque},
    io,
    str::FromStr,
};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    exterior: bool,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Point<T: Copy> {
    x: T,
    y: T,
    z: T,
}

enum Dimension {
    X,
    Y,
    Z,
}

fn cube<T>(input: &str) -> IResult<&str, Point<T>>
where
    T: Copy + FromStr,
{
    map(
        tuple((
            map_res(digit1, str::parse::<T>),
            preceded(tag(","), map_res(digit1, str::parse::<T>)),
            preceded(tag(","), map_res(digit1, str::parse::<T>)),
        )),
        |(x, y, z)| Point { x, y, z },
    )(input)
}

fn bounds<'a, I, T>(i: I, d: Dimension) -> (T, T)
where
    I: Iterator<Item = &'a Point<T>>,
    T: 'a + Copy + PartialOrd,
{
    i.map(|&Point { x, y, z }| match d {
        Dimension::X => x,
        Dimension::Y => y,
        Dimension::Z => z,
    })
    .minmax()
    .into_option()
    .unwrap()
}

fn main() {
    let Args { exterior } = Args::parse();
    let input: Vec<Point<i32>> = io::stdin()
        .lines()
        .map(|line| cube(&line.unwrap()).map(|(_, cube)| cube).unwrap())
        .collect();

    let result = if exterior {
        let (x_min, x_max) = bounds(input.iter(), Dimension::X);
        let (y_min, y_max) = bounds(input.iter(), Dimension::Y);
        let (z_min, z_max) = bounds(input.iter(), Dimension::Z);
        let cubes: HashSet<Point<i32>> = input.into_iter().collect();
        let mut flood: VecDeque<Point<i32>> = VecDeque::new();
        let mut void: HashSet<Point<i32>> = HashSet::new();
        let mut neighbors: Vec<Point<i32>> = Vec::new();
        let origin = Point {
            x: x_min - 1,
            y: y_min - 1,
            z: z_min - 1,
        };
        let mut exterior = 0;
        flood.push_back(origin);
        void.insert(origin);
        while let Some(Point { x, y, z }) = flood.pop_front() {
            neighbors.clear();
            if x > x_min - 1 {
                neighbors.push(Point { x: x - 1, y, z });
            }
            if x < x_max + 1 {
                neighbors.push(Point { x: x + 1, y, z });
            }
            if y > y_min - 1 {
                neighbors.push(Point { x, y: y - 1, z });
            }
            if y < y_max + 1 {
                neighbors.push(Point { x, y: y + 1, z });
            }
            if z > z_min - 1 {
                neighbors.push(Point { x, y, z: z - 1 });
            }
            if z < z_max + 1 {
                neighbors.push(Point { x, y, z: z + 1 });
            }
            for p in neighbors.iter() {
                if cubes.contains(p) {
                    exterior += 1;
                } else if !void.contains(p) {
                    flood.push_back(*p);
                    void.insert(*p);
                }
            }
        }

        exterior
    } else {
        let adjacent = input.iter().enumerate().fold(0, |adjacent, (i, p)| {
            adjacent
                + input[i + 1..].iter().fold(0, |adjacent, q| {
                    adjacent
                        + if (p.x - q.x).abs() + (p.y - q.y).abs() + (p.z - q.z).abs() == 1 {
                            1
                        } else {
                            0
                        }
                })
        });

        input.len() * 6 - adjacent * 2
    };

    println!("{}", result);
}
