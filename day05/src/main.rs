#![warn(clippy::pedantic)]

use itertools::Itertools;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::str::FromStr;

struct Stacks {
    stacks: Vec<VecDeque<char>>,
}

impl Clone for Stacks {
    fn clone(&self) -> Stacks {
        Stacks {
            stacks: self
                .stacks
                .iter()
                .map(|s| s.iter().map(|c| *c).collect::<VecDeque<char>>())
                .collect(),
        }
    }
}

impl Stacks {
    fn get_top_items(&self) -> String {
        self.stacks.iter().map(|stack| stack[0]).collect::<String>()
    }
}

struct Move {
    from: usize,
    to: usize,
    count: usize,
}

impl FromStr for Stacks {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Stacks, Self::Err> {
        let mut lines = input.lines().rev();
        let mut stacks = lines
            .next()
            .ok_or("First line wasn't found in inital stack.")?
            .split_whitespace()
            .map(|_| VecDeque::<char>::new())
            .collect::<Vec<_>>();
        lines
            .map(|line| {
                line.chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .filter(|(_, c)| *c != ' ')
            })
            .flatten()
            .for_each(|(idx, item)| stacks[idx].push_front(item));
        Ok(Stacks { stacks: stacks })
    }
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Move, Self::Err> {
        let (count, from, to) = input
            .split([' '])
            .tuples::<(_, _)>()
            .map(|(_, i)| i.parse::<usize>())
            .collect_tuple::<(_, _, _)>()
            .ok_or("Didn't find three integers in move")?;
        // TODO: actually pass through the error somehow
        Ok(Move {
            from: match from {
                Ok(n) => n - 1,
                Err(_) => return Err("Failed to parse from"),
            },
            to: match to {
                Ok(n) => n - 1,
                Err(_) => return Err("Failed to parse to"),
            },
            count: match count {
                Ok(n) => n,
                Err(_) => return Err("Failed to parse count"),
            },
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("input.txt")?;
    let (input_stacks, input_moves) = data
        .split("\n\n")
        .collect_tuple::<(_, _)>()
        .ok_or("Couldn't split initial stack and moves")?;

    let mut stacks = Stacks::from_str(input_stacks)?;
    let moves = input_moves
        .lines()
        .map(|m| Move::from_str(m))
        .collect::<Result<Vec<_>, _>>()?;

    let mut stacks2 = stacks.clone();
    for i_move in moves.iter() {
        for _ in 1..=i_move.count {
            let item = stacks.stacks[i_move.from]
                .pop_front()
                .ok_or("Invalid move")?;
            stacks.stacks[i_move.to].push_front(item);
        }

        stacks2.stacks[i_move.from]
            .drain(..i_move.count)
            .collect::<VecDeque<_>>()
            .iter()
            .rev()
            .for_each(|item| stacks2.stacks[i_move.to].push_front(*item));
    }

    println!("{}", stacks.get_top_items());
    println!("{}", stacks2.get_top_items());
    Ok(())
}
