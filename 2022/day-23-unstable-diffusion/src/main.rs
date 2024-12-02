use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io,
    ops::{Add, Sub},
};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    complete: bool,
    #[arg(long, default_value_t = false)]
    vis: bool,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Elf<T> {
    current_position: Point<T>,
    proposed_move: Option<Point<T>>,
}

impl<T> Elf<T> {
    fn new(current_position: Point<T>) -> Self {
        Elf {
            current_position,
            proposed_move: None,
        }
    }
}

#[derive(Debug)]
enum Dimension {
    X,
    Y,
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

fn bounds<'a, I, T>(i: I, d: Dimension) -> (T, T)
where
    I: Iterator<Item = &'a Point<T>>,
    T: 'a + Copy + PartialOrd,
{
    i.map(|&Point { x, y }| match d {
        Dimension::X => x,
        Dimension::Y => y,
    })
    .minmax()
    .into_option()
    .unwrap()
}

fn neighbors<T>(s: &HashSet<Point<T>>, &Point { x, y }: &Point<T>) -> HashSet<Direction>
where
    T: Copy + Eq + Hash + Add<i32, Output = T> + Sub<i32, Output = T>,
{
    let mut neighbors = HashSet::new();
    if s.contains(&Point { x, y: y - 1 }) {
        neighbors.insert(Direction::N);
    }
    if s.contains(&Point { x: x + 1, y: y - 1 }) {
        neighbors.insert(Direction::NE);
    }
    if s.contains(&Point { x: x + 1, y }) {
        neighbors.insert(Direction::E);
    }
    if s.contains(&Point { x: x + 1, y: y + 1 }) {
        neighbors.insert(Direction::SE);
    }
    if s.contains(&Point { x, y: y + 1 }) {
        neighbors.insert(Direction::S);
    }
    if s.contains(&Point { x: x - 1, y: y + 1 }) {
        neighbors.insert(Direction::SW);
    }
    if s.contains(&Point { x: x - 1, y }) {
        neighbors.insert(Direction::W);
    }
    if s.contains(&Point { x: x - 1, y: y - 1 }) {
        neighbors.insert(Direction::NW);
    }

    neighbors
}

fn can_move(neighbors: &HashSet<Direction>, d: &Direction) -> bool {
    match d {
        Direction::N => {
            !neighbors.contains(&Direction::N)
                && !neighbors.contains(&Direction::NE)
                && !neighbors.contains(&Direction::NW)
        }
        Direction::E => {
            !neighbors.contains(&Direction::E)
                && !neighbors.contains(&Direction::NE)
                && !neighbors.contains(&Direction::SE)
        }
        Direction::S => {
            !neighbors.contains(&Direction::S)
                && !neighbors.contains(&Direction::SE)
                && !neighbors.contains(&Direction::SW)
        }
        Direction::W => {
            !neighbors.contains(&Direction::W)
                && !neighbors.contains(&Direction::NW)
                && !neighbors.contains(&Direction::SW)
        }
        _ => panic!("invalid move"),
    }
}

fn get_position(&Point { x, y }: &Point<i32>, d: &Direction) -> Point<i32> {
    match d {
        Direction::N => Point { x, y: y - 1 },
        Direction::E => Point { x: x + 1, y },
        Direction::S => Point { x, y: y + 1 },
        Direction::W => Point { x: x - 1, y },
        _ => panic!("invalid move"),
    }
}

fn positions<T>(elves: &[Elf<T>]) -> HashSet<Point<T>>
where
    T: Copy + Eq + Hash,
{
    elves.iter().map(|elf| elf.current_position).collect()
}

fn map(elves: &[Elf<i32>]) -> Vec<String> {
    let positions = positions(elves);
    let (x_min, x_max) = bounds(positions.iter(), Dimension::X);
    let (y_min, y_max) = bounds(positions.iter(), Dimension::Y);
    (y_min..=y_max)
        .map(|y| {
            (x_min..=x_max)
                .map(|x| {
                    if positions.contains(&Point { x, y }) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect()
}

fn main() {
    let Args { complete, vis } = Args::parse();
    let mut elves = io::stdin()
        .lines()
        .enumerate()
        .fold(Vec::new(), |mut elves, (row, line)| {
            line.unwrap().chars().enumerate().for_each(|(col, char)| {
                if char == '#' {
                    elves.push(Elf::new(Point {
                        x: col as i32,
                        y: row as i32,
                    }));
                }
            });
            elves
        });
    let mut round = 0;

    for (a, b, c, d) in [Direction::N, Direction::S, Direction::W, Direction::E]
        .iter()
        .cycle()
        .tuple_windows()
    {
        let current_positions = positions(&elves);

        round += 1;
        elves.iter_mut().for_each(
            |Elf {
                 current_position,
                 proposed_move,
             }| {
                let neighbors = neighbors(&current_positions, current_position);
                if neighbors.is_empty() {
                    *proposed_move = None;
                } else if can_move(&neighbors, a) {
                    *proposed_move = Some(get_position(current_position, a));
                } else if can_move(&neighbors, b) {
                    *proposed_move = Some(get_position(current_position, b));
                } else if can_move(&neighbors, c) {
                    *proposed_move = Some(get_position(current_position, c));
                } else if can_move(&neighbors, d) {
                    *proposed_move = Some(get_position(current_position, d));
                }
            },
        );

        let proposed_moves = elves.iter().fold(
            HashMap::new(),
            |mut proposed_moves, Elf { proposed_move, .. }| {
                if let Some(proposed_move) = proposed_move {
                    proposed_moves
                        .entry(*proposed_move)
                        .and_modify(|n| *n += 1)
                        .or_insert(1);
                }

                proposed_moves
            },
        );

        if proposed_moves.is_empty() {
            break;
        }

        elves.iter_mut().for_each(
            |Elf {
                 current_position,
                 proposed_move,
             }| {
                if let Some(proposed_move) = proposed_move {
                    if *proposed_moves.get(proposed_move).unwrap() == 1 {
                        *current_position = *proposed_move;
                    }
                }
            },
        );

        if !complete && round == 10 {
            break;
        }
    }

    let map = map(&elves);
    if vis {
        println!("{}", map.join("\n"));
    }
    let result = if complete {
        round
    } else {
        map.iter().map(|row| row.matches('.').count()).sum()
    };

    println!("{}", result);
}
