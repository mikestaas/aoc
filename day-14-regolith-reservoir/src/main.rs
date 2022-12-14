use clap::Parser;
use itertools::{Either, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{cmp, collections::BTreeMap, io};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    floor: bool,
    #[arg(long, default_value_t = false)]
    vis: bool,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

enum Dimension {
    X,
    Y,
}

fn path(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(
        tag(" -> "),
        map(
            separated_pair(
                map_res(digit1, str::parse::<usize>),
                tag(","),
                map_res(digit1, str::parse::<usize>),
            ),
            |(x, y)| Point { x, y },
        ),
    )(input)
}

fn draw((p1, p2): (&Point, &Point)) -> impl Iterator<Item = Point> {
    let &Point { x, y } = p1;
    let x_min = cmp::min(p1.x, p2.x);
    let x_max = cmp::max(p1.x, p2.x);
    let y_min = cmp::min(p1.y, p2.y);
    let y_max = cmp::max(p1.y, p2.y);
    match (x_max - x_min, y_max - y_min) {
        (_, 0) => Either::Left((x_min..=x_max).map(move |x| Point { x, y })),
        (0, _) => Either::Right((y_min..=y_max).map(move |y| Point { x, y })),
        (_, _) => panic!("diagonal draw"),
    }
}

fn drop(
    x: usize,
    map: &BTreeMap<Point, char>,
    can_move: impl Fn(Point, &BTreeMap<Point, char>, bool) -> bool,
) -> Option<Point> {
    let mut p = Point { x, y: 0 };
    loop {
        let down = Point { x: p.x, y: p.y + 1 };
        let down_left = Point {
            x: p.x - 1,
            y: p.y + 1,
        };
        let down_right = Point {
            x: p.x + 1,
            y: p.y + 1,
        };
        if can_move(down, map, false) {
            p = down;
        } else if can_move(down_left, map, false) {
            p = down_left;
        } else if can_move(down_right, map, false) {
            p = down_right;
        } else {
            break;
        }
        if !can_move(p, map, true) {
            break;
        }
    }
    if can_move(p, map, true) {
        Some(p)
    } else {
        None
    }
}

fn bounds<'a, I>(i: I, d: Dimension) -> (usize, usize)
where
    I: Iterator<Item = &'a Point>,
{
    i.map(|&Point { x, y }| match d {
        Dimension::X => x,
        Dimension::Y => y,
    })
    .minmax()
    .into_option()
    .unwrap()
}

fn main() {
    let Args { floor, vis } = Args::parse();
    let input: Vec<Vec<Point>> = io::stdin()
        .lines()
        .map(|line| path(&line.unwrap()).map(|(_, path)| path).unwrap())
        .collect();
    let mut map: BTreeMap<Point, char> = BTreeMap::new();
    input.iter().for_each(|path| {
        path.iter().tuple_windows().map(draw).for_each(|line| {
            line.for_each(|point| {
                map.insert(point, '#');
            });
        });
    });
    let (x_min, x_max) = bounds(input.iter().flatten(), Dimension::X);
    let (_, y_max) = bounds(input.iter().flatten(), Dimension::Y);
    let can_move = |p: Point, map: &BTreeMap<Point, char>, check_bounds: bool| {
        if floor {
            p.y < y_max + 2 && map.get(&p).is_none()
        } else if check_bounds {
            p.x >= x_min && p.x <= x_max && p.y <= y_max
        } else {
            map.get(&p).is_none()
        }
    };
    let mut result = 0;
    while let Some(point) = drop(500, &map, can_move) {
        map.insert(point, 'o');
        result += 1;
    }
    if vis {
        let (x_min, x_max) = if floor {
            bounds(map.keys(), Dimension::X)
        } else {
            (x_min, x_max)
        };
        let y_max = if floor { y_max + 1 } else { y_max };
        for y in 0..=y_max {
            for x in x_min..=x_max {
                if let Some(c) = map.get(&Point { x, y }) {
                    print!("{}", c);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        if floor {
            for _ in x_min..=x_max {
                print!("#");
            }
            println!();
        }
    }
    println!("{}", result);
}
