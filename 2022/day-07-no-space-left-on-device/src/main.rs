use clap::Parser;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, rest},
    sequence::{pair, preceded, separated_pair},
    IResult,
};
use std::io;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    smallest: bool,
}

#[derive(Debug)]
enum Command<'a> {
    ChangeDirectory { name: &'a str },
    Invalid,
    List,
}

#[derive(Debug)]
struct Directory<'a> {
    name: &'a str,
    directories: Vec<usize>,
    files: Vec<usize>,
    parent: usize,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str) -> Self {
        Directory {
            name,
            directories: Vec::new(),
            files: Vec::new(),
            parent: 0,
        }
    }
}

fn command(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("$ "),
            alt((
                pair(tag("ls"), tag("")),
                separated_pair(tag("cd"), tag(" "), rest),
            )),
        ),
        |(cmd, arg)| match (cmd, arg) {
            ("cd", name) => Command::ChangeDirectory { name },
            ("ls", _) => Command::List,
            _ => Command::Invalid,
        },
    )(input)
}

fn directory(input: &str) -> IResult<&str, Directory> {
    map(preceded(tag("dir "), rest), Directory::new)(input)
}

fn file(input: &str) -> IResult<&str, usize> {
    map(
        separated_pair(digit1, tag(" "), rest),
        |(size, _name): (&str, &str)| size.parse().unwrap(),
    )(input)
}

fn main() {
    let Args { smallest } = Args::parse();
    let input: Vec<String> = io::stdin().lines().map(Result::unwrap).collect();
    let mut directories = vec![Directory::new("/")];
    let mut current_idx: usize = 0;

    for line in input.iter() {
        if let Ok((_, Command::ChangeDirectory { name })) = command(line) {
            current_idx = directories
                .get(current_idx)
                .map(|dir| match name {
                    "/" => 0,
                    ".." => dir.parent,
                    _ => *dir
                        .directories
                        .iter()
                        .find(|&idx| {
                            directories
                                .get(*idx)
                                .map(|dir| dir.name == name)
                                .unwrap_or(false)
                        })
                        .unwrap_or(&current_idx),
                })
                .unwrap();
        }

        if let Ok((_, mut directory)) = directory(line) {
            directory.parent = current_idx;
            let idx = directories.len();
            directories.push(directory);
            if let Some(dir) = directories.get_mut(current_idx) {
                dir.directories.push(idx);
            }
        }

        if let Ok((_, file)) = file(line) {
            if let Some(dir) = directories.get_mut(current_idx) {
                dir.files.push(file);
            }
        }
    }

    fn size(directories: &Vec<Directory>, idx: usize) -> usize {
        directories
            .get(idx)
            .map(|dir| {
                dir.files.iter().sum::<usize>()
                    + dir
                        .directories
                        .iter()
                        .fold(0, |total, idx| total + size(directories, *idx))
            })
            .unwrap_or(0)
    }

    fn filter_by_size<'a, F: FnMut(&usize) -> bool + 'a>(
        directories: &'a Vec<Directory<'a>>,
        f: F,
    ) -> impl Iterator<Item = usize> + 'a {
        (0..directories.len())
            .map(move |idx| size(directories, idx))
            .filter(f)
    }

    let result: usize = if smallest {
        let free = 70000000 - size(&directories, 0);
        filter_by_size(&directories, |size: &usize| *size >= 30000000 - free)
            .min()
            .unwrap()
    } else {
        filter_by_size(&directories, |size: &usize| *size <= 100000).sum()
    };

    println!("{}", result);
}
