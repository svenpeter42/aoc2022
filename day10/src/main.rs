#![warn(clippy::pedantic)]

use std::error::Error;
use std::fs;
use std::iter;
use std::str::FromStr;

struct CPUState {
    x: isize,
}

trait CPUInstruction {
    fn run(&self, state: &mut CPUState) -> usize;
    fn parse<'a>(
        tokens: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Box<dyn CPUInstruction>, Box<dyn Error>>
    where
        Self: Sized;
}

struct NOP {}

struct ADDX {
    n: isize,
}

impl CPUInstruction for NOP {
    fn run(&self, _state: &mut CPUState) -> usize {
        1
    }

    fn parse<'a>(
        _: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Box<dyn CPUInstruction>, Box<dyn Error>> {
        Ok(Box::new(NOP {}))
    }
}

impl CPUInstruction for ADDX {
    fn run(&self, state: &mut CPUState) -> usize {
        state.x += self.n;
        2
    }

    fn parse<'a>(
        tokens: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Box<dyn CPUInstruction>, Box<dyn Error>> {
        Ok(Box::new(ADDX {
            n: tokens
                .next()
                .ok_or("addx without imm value")?
                .parse::<isize>()?,
        }))
    }
}

impl FromStr for Box<dyn CPUInstruction> {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Box<dyn CPUInstruction>, Box<dyn Error>> {
        let mut tokens = s.split(" ");
        let instr = tokens.next().ok_or("empty input line")?;

        match instr {
            "noop" => NOP::parse(&mut tokens),
            "addx" => ADDX::parse(&mut tokens),
            _ => Err(format!("unknown instruction {}", instr))?,
        }
    }
}

fn parse_instructions(s: &str) -> Result<Vec<Box<dyn CPUInstruction>>, Box<dyn Error>> {
    s.lines()
        .map(|line| line.parse::<Box<dyn CPUInstruction>>())
        .collect::<Result<_, _>>()
}

fn run_cpu<'a>(
    instrs: &'a Vec<Box<dyn CPUInstruction>>,
) -> impl Iterator<Item = (usize, isize)> + 'a {
    let mut cpu = CPUState { x: 1 };
    instrs
        .iter()
        .map(move |instr| iter::repeat(cpu.x).take(instr.run(&mut cpu)))
        .flatten()
        .enumerate()
}

fn calc_sum(instrs: &Vec<Box<dyn CPUInstruction>>) -> isize {
    run_cpu(instrs)
        .skip(19)
        .step_by(40)
        .map(|(pc, x)| ((pc + 1) as isize) * x)
        .sum::<isize>()
}

fn draw_screen(instrs: &Vec<Box<dyn CPUInstruction>>) -> Vec<char> {
    run_cpu(instrs)
        .map(|(pc, x)| match ((pc % 40) as isize) - x {
            1 => '#',
            0 => '#',
            -1 => '#',
            _ => '.',
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = parse_instructions(&fs::read_to_string("input.txt")?)?;

    println!("A: {}", calc_sum(&instructions));

    println!("B:");
    for row in draw_screen(&instructions).chunks(40) {
        println!("{}", row.iter().collect::<String>());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &'static str = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop\n";

    #[test]
    fn test_sum() {
        let instructions = parse_instructions(EXAMPLE_A).unwrap();

        assert_eq!(calc_sum(&instructions), 13140);
    }

    #[test]
    fn test_screen() {
        let instructions = parse_instructions(EXAMPLE_A).unwrap();

        assert_eq!(draw_screen(&instructions).iter().collect::<String>(), "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....");
    }
}
