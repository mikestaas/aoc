use clap::Parser;
use std::{io, str::FromStr};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    proper: bool,
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

struct ParseError;

impl std::str::FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scissors),
            _ => Err(ParseError),
        }
    }
}

enum Strategy {
    Lose,
    Draw,
    Win,
}

trait MoveFor {
    fn move_for(&self, oponent_move: &Move) -> Move;
}

impl std::str::FromStr for Strategy {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Strategy::Lose),
            "Y" => Ok(Strategy::Draw),
            "Z" => Ok(Strategy::Win),
            _ => Err(ParseError),
        }
    }
}

impl MoveFor for Strategy {
    fn move_for(&self, oponent_move: &Move) -> Move {
        match (oponent_move, self) {
            (Move::Rock, Strategy::Lose) => Move::Scissors,
            (Move::Rock, Strategy::Draw) => Move::Rock,
            (Move::Rock, Strategy::Win) => Move::Paper,
            (Move::Paper, Strategy::Lose) => Move::Rock,
            (Move::Paper, Strategy::Draw) => Move::Paper,
            (Move::Paper, Strategy::Win) => Move::Scissors,
            (Move::Scissors, Strategy::Lose) => Move::Paper,
            (Move::Scissors, Strategy::Draw) => Move::Scissors,
            (Move::Scissors, Strategy::Win) => Move::Rock,
        }
    }
}

fn strategy(opponent_move: &Move, suggested_play: &str, proper: bool) -> Result<Move, ParseError> {
    if proper {
        Strategy::from_str(suggested_play).map(|strategy| strategy.move_for(opponent_move))
    } else {
        Move::from_str(suggested_play)
    }
}

fn main() {
    let Args { proper } = Args::parse();
    let input = io::stdin().lines();
    let score = input.fold(0, |mut score, line| {
        if let Some((opponent_move, suggested_play)) = line.unwrap().split_once(' ') {
            if let Ok(opponent_move) = Move::from_str(opponent_move) {
                if let Ok(suggested_play) = strategy(&opponent_move, suggested_play, proper) {
                    score += match (opponent_move, suggested_play) {
                        (Move::Rock, Move::Rock) => 4,         // 1 for rock, 3 for draw
                        (Move::Rock, Move::Paper) => 8,        // 2 for paper, 6 for win
                        (Move::Rock, Move::Scissors) => 3,     // 3 for scisors, 0 for loss
                        (Move::Paper, Move::Rock) => 1,        // 1 for rock, 0 for loss
                        (Move::Paper, Move::Paper) => 5,       // 2 for paper, 3 for draw
                        (Move::Paper, Move::Scissors) => 9,    // 3 for scisors, 6 for win
                        (Move::Scissors, Move::Rock) => 7,     // 1 for rock, 6 for win
                        (Move::Scissors, Move::Paper) => 2,    // 2 for paper, 0 for loss
                        (Move::Scissors, Move::Scissors) => 6, // 3 for scisors, 3 for draw
                    }
                }
            }
        }
        score
    });
    println!("{}", score);
}
