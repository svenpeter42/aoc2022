#![warn(clippy::pedantic)]

use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Goal {
    Loose,
    Draw,
    Win,
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Move, Self::Err> {
        match input {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scissors),
            _ => Err("Invalid move"),
        }
    }
}

impl FromStr for Goal {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Goal, Self::Err> {
        match input {
            "X" => Ok(Goal::Loose),
            "Y" => Ok(Goal::Draw),
            "Z" => Ok(Goal::Win),
            _ => Err("Invalid goal"),
        }
    }
}

fn calc_winloss_score(x: Move, y: Move) -> usize {
    match (x, y) {
        (Move::Rock, Move::Rock) => 3,
        (Move::Paper, Move::Paper) => 3,
        (Move::Scissors, Move::Scissors) => 3,
        (Move::Rock, Move::Paper) => 6,
        (Move::Paper, Move::Rock) => 0,
        (Move::Rock, Move::Scissors) => 0,
        (Move::Scissors, Move::Rock) => 6,
        (Move::Paper, Move::Scissors) => 6,
        (Move::Scissors, Move::Paper) => 0,
    }
}

fn calc_score(x: Move) -> usize {
    match x {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn get_winning_move(x: Move) -> Move {
    match x {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock,
    }
}

fn get_loosing_move(x: Move) -> Move {
    match x {
        Move::Rock => Move::Scissors,
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
    }
}

fn get_move(x: Move, goal: Goal) -> Move {
    match goal {
        Goal::Win => get_winning_move(x),
        Goal::Draw => x,
        Goal::Loose => get_loosing_move(x),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let score: usize = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| {
            let moves = line.split(" ").collect::<Vec<&str>>();
            let x = Move::from_str(moves[0]).unwrap();
            let y = Move::from_str(moves[1]).unwrap();
            calc_score(y) + calc_winloss_score(x, y)
        })
        .sum();
    println!("A: {score}");

    let score2: usize = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| {
            let moves = line.split(" ").collect::<Vec<&str>>();
            let x = Move::from_str(moves[0]).unwrap();
            let goal = Goal::from_str(moves[1]).unwrap();
            let y = get_move(x, goal);
            calc_score(y) + calc_winloss_score(x, y)
        })
        .sum();
    println!("B: {score2}");

    Ok(())
}
