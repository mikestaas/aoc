use clap::Parser;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, opt, recognize},
    sequence::{preceded, tuple},
    IResult,
};
use std::{
    cmp,
    collections::{BTreeMap, BTreeSet},
    io,
    ops::RangeInclusive,
    str::FromStr,
};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = 10)]
    depth: i32,
    #[arg(long, default_value_t = false)]
    tune: bool,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }

    fn distance(&self, b: &Point) -> i32 {
        (self.x - b.x).abs() + (self.y - b.y).abs()
    }

    fn find_nearest<'a>(&self, i: impl Iterator<Item = &'a Self>) -> Option<&'a Self> {
        i.min_by(|&a, &b| self.distance(a).cmp(&self.distance(b)))
    }
}

fn signed<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(preceded(opt(tag("-")), digit1)), str::parse::<T>)(input)
}

fn data(input: &str) -> IResult<&str, (Point, Point)> {
    tuple((
        map(
            tuple((
                preceded(tag("Sensor at x="), signed::<i32>),
                preceded(tag(", y="), signed::<i32>),
            )),
            Point::new,
        ),
        map(
            tuple((
                preceded(tag(": closest beacon is at x="), signed::<i32>),
                preceded(tag(", y="), signed::<i32>),
            )),
            Point::new,
        ),
    ))(input)
}

fn x_range(Point { x, y }: Point, distance: i32, y_intercept: i32) -> Option<RangeInclusive<i32>> {
    let dy = (y - y_intercept).abs();
    if dy < distance {
        let dx = distance - dy;
        Some(x - dx..=x + dx)
    } else {
        None
    }
}

fn merge<T>(ranges: &[RangeInclusive<T>]) -> Vec<RangeInclusive<T>>
where
    T: Ord + Clone,
{
    let mut ranges = ranges.to_vec();
    ranges.sort_unstable_by_key(|range| range.start().clone());
    ranges.iter().fold(Vec::new(), move |mut merged, range| {
        let last = merged.last();
        if last.is_none() || last.unwrap().end() < range.start() {
            merged.push(range.clone());
        } else {
            let last = merged.pop().unwrap();
            merged.push(last.start().clone()..=cmp::max(last.end().clone(), range.end().clone()))
        }

        merged
    })
}

fn sum(ranges: &[RangeInclusive<i32>]) -> i32 {
    ranges
        .iter()
        .fold(0, |sum, range| sum + range.end() - range.start())
}

fn main() {
    let Args { depth, tune } = Args::parse();
    let input: Vec<(Point, Point)> = io::stdin()
        .lines()
        .map(|line| data(&line.unwrap()).map(|(_, data)| data).unwrap())
        .collect();
    let beacons: BTreeSet<Point> = input.iter().map(|&(_, beacon)| beacon).collect();
    let sensors: BTreeMap<Point, i32> = input
        .iter()
        .flat_map(|&(sensor, _)| {
            sensor
                .find_nearest(beacons.iter())
                .map(|beacon| (sensor, sensor.distance(beacon)))
        })
        .collect();

    let result = if tune {
        (0..=depth * 2)
            .find_map(|y| {
                let ranges = merge(
                    &sensors
                        .iter()
                        .flat_map(|(&point, &distance)| x_range(point, distance, y))
                        .collect::<Vec<RangeInclusive<i32>>>(),
                );
                if ranges.len() > 1 {
                    Some((ranges[0].end() + 1) as u64 * 4000000u64 + y as u64)
                } else {
                    None
                }
            })
            .unwrap()
    } else {
        sum(&merge(
            &sensors
                .iter()
                .flat_map(|(&point, &distance)| x_range(point, distance, depth))
                .collect::<Vec<RangeInclusive<i32>>>(),
        )) as u64
    };
    println!("{}", result);
}
