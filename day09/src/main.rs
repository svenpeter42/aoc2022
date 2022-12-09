#![warn(clippy::pedantic)]

use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::str::FromStr;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Move {
    count: usize,
    dir: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

struct Rope {
    heads: Vec<Pos>,
}

impl Rope {
    fn new(n_heads: usize) -> Rope {
        Rope {
            heads: (0..n_heads).map(|_| Pos { x: 0, y: 0 }).collect(),
        }
    }

    fn get_tail_position(&self) -> Option<&Pos> {
        self.heads.last()
    }

    fn update_tail(head: Pos, tail: &mut Pos) {
        let dx = tail.x - head.x;
        let dy = tail.y - head.y;
        if dx.abs() < 2 && dy.abs() < 2 {
            // head and tail touch
        } else {
            tail.x -= dx.signum();
            tail.y -= dy.signum();
        }
    }

    fn move_head(&mut self, d: &Direction) {
        match d {
            Direction::Up => self.heads[0].y += 1,
            Direction::Down => self.heads[0].y -= 1,
            Direction::Right => self.heads[0].x += 1,
            Direction::Left => self.heads[0].x -= 1,
        }

        for i in 1..self.heads.len() {
            Rope::update_tail(self.heads[i - 1], &mut self.heads[i]);
        }
    }
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            d => Err(format!("Invalid direction: {}", d))?,
        }
    }
}

impl FromStr for Move {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Move, Self::Err> {
        match input.split(" ").collect_tuple::<(_, _)>() {
            Some((dir, count)) => Ok(Move {
                count: count.parse::<usize>()?,
                dir: Direction::from_str(dir)?,
            }),
            None => Err("Invalid input line: couldn't split")?,
        }
    }
}

fn parse_moves(input: &str) -> Result<Vec<Move>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| Move::from_str(line))
        .collect::<Result<Vec<_>, _>>()
}

fn count_unqiue_tail_positions(moves: &Vec<Move>, n_heads: usize) -> usize {
    let mut rope = Rope::new(n_heads);
    let mut positions = HashSet::<Pos>::new();

    for m in moves.iter() {
        for _ in 0..m.count {
            rope.move_head(&m.dir);
            positions.insert(*rope.get_tail_position().unwrap());
        }
    }

    positions.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let moves = parse_moves(&fs::read_to_string("input.txt")?)?;

    println!("A: {}", count_unqiue_tail_positions(&moves, 2));
    println!("B: {}", count_unqiue_tail_positions(&moves, 10));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &'static str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    const EXAMPLE_B: &'static str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

    #[test]
    fn example_a() {
        assert_eq!(
            count_unqiue_tail_positions(&parse_moves(EXAMPLE_A).unwrap(), 2),
            13
        );
        assert_eq!(
            count_unqiue_tail_positions(&parse_moves(EXAMPLE_A).unwrap(), 10),
            1
        );
    }

    #[test]
    fn example_b() {
        assert_eq!(
            count_unqiue_tail_positions(&parse_moves(EXAMPLE_B).unwrap(), 10),
            36
        );
    }
}
