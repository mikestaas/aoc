use clap::Parser;
use std::io;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    score: bool,
}

fn main() {
    let Args { score } = Args::parse();
    let input: Vec<Vec<u32>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().flat_map(|c| c.to_digit(10)).collect())
        .collect();
    let width = input.iter().map(|row| row.len()).max().unwrap();
    let depth = input.len();

    let result: usize = if score {
        let last_row = depth - 1;
        let last_col = width - 1;
        (1..last_row)
            .flat_map(|y| {
                (1..last_col)
                    .map(|x| {
                        let height = input[y][x];
                        let blocks_view = |x: usize, y: usize| input[y][x] >= height;
                        let plus_one = |x: usize| x + 1;

                        (0..x)
                            .rev()
                            .position(|x| blocks_view(x, y))
                            .map_or(x, plus_one)
                            * (x + 1..width)
                                .position(|x| blocks_view(x, y))
                                .map_or(last_col - x, plus_one)
                            * (0..y)
                                .rev()
                                .position(|y| blocks_view(x, y))
                                .map_or(y, plus_one)
                            * (y + 1..depth)
                                .position(|y| blocks_view(x, y))
                                .map_or(last_row - y, plus_one)
                    })
                    .collect::<Vec<usize>>()
            })
            .max()
            .unwrap()
    } else {
        (0..depth)
            .map(|y| {
                (0..width)
                    .filter(|&x| {
                        if x == 0 || x == width - 1 || y == 0 || y == depth - 1 {
                            true
                        } else {
                            let height = input[y][x];
                            let is_visible = |x: usize, y: usize| input[y][x] < height;

                            (0..x).all(|x| is_visible(x, y))
                                || (x + 1..width).all(|x| is_visible(x, y))
                                || (0..y).all(|y| is_visible(x, y))
                                || (y + 1..depth).all(|y| is_visible(x, y))
                        }
                    })
                    .count()
            })
            .sum()
    };

    println!("{}", result)
}
