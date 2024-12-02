use clap::Parser;
use lending_iterator::{lending_iterator::constructors::windows_mut, LendingIterator};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashSet, io};

#[derive(Parser)]
struct Args {
    #[arg(default_value_t = 2)]
    len: usize,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_up(&mut self) {
        self.y += 1;
    }
    fn move_down(&mut self) {
        self.y -= 1;
    }
    fn move_left(&mut self) {
        self.x -= 1;
    }
    fn move_right(&mut self) {
        self.x += 1;
    }
    fn compare(&self, p: &Point) -> Point {
        Point {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }
}

#[derive(Debug)]
enum Motion {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

fn motion(input: &str) -> IResult<&str, Motion> {
    map(
        separated_pair(alpha1, tag(" "), digit1),
        |(direction, count): (&str, &str)| {
            let count = str::parse::<u32>(count).unwrap();
            match direction {
                "U" => Motion::Up(count),
                "D" => Motion::Down(count),
                "L" => Motion::Left(count),
                "R" => Motion::Right(count),
                _ => panic!("{} not a valid motion", direction),
            }
        },
    )(input)
}

fn main() {
    let Args { len } = Args::parse();
    let input: Vec<Motion> = io::stdin()
        .lines()
        .map(|line| motion(&line.unwrap()).map(|(_, motion)| motion).unwrap())
        .collect();
    let mut visited: HashSet<Point> = HashSet::new();
    let mut snake = vec![Point { x: 0, y: 0 }; len];
    let mut moves = |count: u32, f: fn(&mut Point)| {
        for _ in 0..count {
            f(&mut snake[0]);
            snake.windows_mut::<2>().for_each(|[head, tail]| {
                match head.compare(tail) {
                    Point { x: 0, y: 0 } => {}
                    Point { x: 0, y } => {
                        if y > 1 {
                            tail.move_up();
                        } else if y < -1 {
                            tail.move_down();
                        }
                    }
                    Point { x, y: 0 } => {
                        if x > 1 {
                            tail.move_right();
                        } else if x < -1 {
                            tail.move_left();
                        }
                    }
                    Point { x, y } => {
                        if x > 0 && y > 1 || x > 1 && y > 0 {
                            tail.move_up();
                            tail.move_right();
                        } else if x < 0 && y > 1 || x < -1 && y > 0 {
                            tail.move_up();
                            tail.move_left();
                        } else if x > 0 && y < -1 || x > 1 && y < 0 {
                            tail.move_down();
                            tail.move_right();
                        } else if x < 0 && y < -1 || x < -1 && y < 0 {
                            tail.move_down();
                            tail.move_left();
                        }
                    }
                }
            });
            visited.insert(snake.last().unwrap().clone());
        }
    };

    for motion in &input {
        match motion {
            Motion::Up(count) => moves(*count, |p: &mut Point| p.move_up()),
            Motion::Down(count) => moves(*count, |p: &mut Point| p.move_down()),
            Motion::Left(count) => moves(*count, |p: &mut Point| p.move_left()),
            Motion::Right(count) => moves(*count, |p: &mut Point| p.move_right()),
        }
    }

    println!("{}", visited.len());
}
