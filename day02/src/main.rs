use std::error::Error;
use std::fs;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

const LOOSE: u32 = 1;
const DRAW: u32 = 2;
const WIN: u32 = 3;

fn parse_move(x: &str) -> Result<u32, &'static str> {
    match x {
        "A" => Ok(ROCK),
        "B" => Ok(PAPER),
        "C" => Ok(SCISSORS),
        "X" => Ok(ROCK),
        "Y" => Ok(PAPER),
        "Z" => Ok(SCISSORS),
        _ => Err("Invalid move"),
    }
}

fn parse_goal(x: &str) -> Result<u32, &'static str> {
    match x {
        "X" => Ok(LOOSE),
        "Y" => Ok(DRAW),
        "Z" => Ok(WIN),
        _ => Err("Invalid goal"),
    }
}

fn calc_score(x: u32, y: u32) -> Result<u32, &'static str> {
    match (x, y) {
        (ROCK, ROCK) => Ok(3),
        (PAPER, PAPER) => Ok(3),
        (SCISSORS, SCISSORS) => Ok(3),
        (ROCK, PAPER) => Ok(6),
        (PAPER, ROCK) => Ok(0),
        (ROCK, SCISSORS) => Ok(0),
        (SCISSORS, ROCK) => Ok(6),
        (PAPER, SCISSORS) => Ok(6),
        (SCISSORS, PAPER) => Ok(0),
        _ => Err("Invalid moves to calc score"),
    }
}

fn get_move(x: u32, goal: u32) -> Result<u32, &'static str> {
    match (x, goal) {
        (WIN, ROCK) => Ok(PAPER),
        (WIN, PAPER) => Ok(SCISSORS),
        (WIN, SCISSORS) => Ok(ROCK),
        (DRAW, ROCK) => Ok(ROCK),
        (DRAW, PAPER) => Ok(PAPER),
        (DRAW, SCISSORS) => Ok(SCISSORS),
        (LOOSE, ROCK) => Ok(SCISSORS),
        (LOOSE, PAPER) => Ok(ROCK),
        (LOOSE, SCISSORS) => Ok(PAPER),
        _ => Err("Invalid inputs to get move"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let score: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| {
            let moves = line.split(" ").collect::<Vec<&str>>();
            let x = parse_move(moves[0]).unwrap();
            let y = parse_move(moves[1]).unwrap();
            let result_score = calc_score(x, y).unwrap();
            result_score + y
        })
        .sum();
    println!("A: {score}");

    let score2: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| {
            let moves = line.split(" ").collect::<Vec<&str>>();
            let x = parse_move(moves[0]).unwrap();
            let goal = parse_goal(moves[1]).unwrap();
            let y = get_move(x, goal).unwrap();
            let result_score = calc_score(x, y).unwrap();
            result_score + y
        })
        .sum();
    println!("B: {score2}");
    Ok(())
}
