use clap::Parser;
use colored::Colorize;
use std::{collections::HashMap, io};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    any: bool,
    #[arg(long, default_value_t = false)]
    vis: bool,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn find_points(vec: &[Vec<char>], target: char) -> Vec<Point> {
    vec.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| {
                if *c == target {
                    Some(Point { x, y })
                } else {
                    None
                }
            })
        })
        .collect()
}

fn find_and_replace_point(vec: &mut [Vec<char>], target: char, replacement: char) -> Option<Point> {
    vec.iter_mut().enumerate().find_map(|(y, row)| {
        row.iter_mut().enumerate().find_map(|(x, c)| {
            if *c == target {
                *c = replacement;

                Some(Point { x, y })
            } else {
                None
            }
        })
    })
}

fn one_lower_or_above(current: char, next: char) -> bool {
    current as i32 - next as i32 <= 1
}

fn find_moves(
    vec: &[Vec<char>],
    &Point { x, y }: &Point,
    predicate: fn(char, char) -> bool,
) -> Vec<Point> {
    let current = vec[y][x];
    let mut result = Vec::new();
    let can_move = |dx: i32, dy: i32| -> Option<Point> {
        if y > 0 || dy >= 0 && x > 0 || dx >= 0 {
            let y = (y as i32 + dy) as usize;
            let x = (x as i32 + dx) as usize;
            if let Some(&next) = vec.get(y).and_then(|row| row.get(x)) {
                if predicate(current, next) {
                    return Some(Point { x, y });
                }
            }
        }

        None
    };
    if let Some(point) = can_move(0, -1) {
        result.push(point);
    }
    if let Some(point) = can_move(-1, 0) {
        result.push(point);
    }
    if let Some(point) = can_move(1, 0) {
        result.push(point);
    }
    if let Some(point) = can_move(0, 1) {
        result.push(point);
    }

    result
}

fn main() {
    let Args { any, vis } = Args::parse();
    let mut input: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let start = find_and_replace_point(&mut input, 'S', 'a').unwrap();
    let potential_start = if any {
        find_points(&input, 'a')
    } else {
        vec![start]
    };
    let end = find_and_replace_point(&mut input, 'E', 'z').unwrap();
    let mut visited: HashMap<Point, u32> = HashMap::new();
    visited.insert(end, 0);

    while potential_start
        .iter()
        .all(|point| visited.get(point).is_none())
    {
        let mut new_points: HashMap<Point, u32> = HashMap::new();
        for (&point, &distance) in visited.iter() {
            for next in find_moves(&input, &point, one_lower_or_above) {
                new_points.insert(next, distance + 1);
            }
        }
        for (&point, &distance) in new_points.iter() {
            visited.entry(point).or_insert(distance);
        }
    }

    if vis {
        let colors: [(u8, u8, u8); 6] = [
            (127, 0, 127),
            (0, 0, 255),
            (0, 255, 0),
            (255, 255, 0),
            (255, 127, 0),
            (255, 0, 0),
        ];
        input.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, c)| {
                if let Some(distance) = visited.get(&Point { x, y }) {
                    let (r, g, b) = colors[(distance % 6) as usize];
                    print!("{}", c.to_string().truecolor(r, g, b));
                } else {
                    print!("{}", c);
                }
            });
            println!();
        });
    } else {
        let result: &u32 = potential_start
            .iter()
            .filter_map(|point| visited.get(point))
            .min()
            .unwrap();

        println!("{}", result);
    }
}
