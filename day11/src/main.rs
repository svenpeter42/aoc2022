#![warn(clippy::pedantic)]

use itertools::Itertools;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Op {
    Add(usize),
    Mul(usize),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    test_div: usize,
    target_true: usize,
    target_false: usize,
    n_inspected: usize,
}

impl FromStr for Op {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Op, Box<dyn Error>> {
        if s == "new = old * old" {
            Ok(Op::Square)
        } else if let Some(val) = s.split(" * ").nth(1) {
            Ok(Op::Mul(val.parse().map_err(|e| {
                format!("Failed to parse Op::Mul `{}` with {:?}", s, e)
            })?))
        } else if let Some(val) = s.split(" + ").nth(1) {
            Ok(Op::Add(val.parse().map_err(|e| {
                format!("Failed to parse Op::Add `{}` with {:?}", s, e)
            })?))
        } else {
            return Err(format!("Invalid operation: {}", s).into());
        }
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Monkey, Box<dyn Error>> {
        let (_, line_items, line_op, line_test, line_true, line_false) = s
            .lines()
            .collect_tuple::<_>()
            .ok_or("Invalid number of lines for Monkey")?;
        Ok(Monkey {
            items: line_items
                .split("Starting items: ")
                .nth(1)
                .ok_or(format!("Invalid items line: {}", line_items))?
                .split(", ")
                .map(str::parse)
                .collect::<Result<VecDeque<_>, _>>()
                .map_err(|e| format!("Failed to parse items `{}`: {:?}", line_items, e))?,
            op: Op::from_str(
                line_op
                    .split("Operation: ")
                    .nth(1)
                    .ok_or(format!("Invalid operation line: {}", line_op))?,
            )?,
            test_div: line_test
                .split("Test: divisible by ")
                .nth(1)
                .ok_or(format!("Invalid test line: {}", line_test))?
                .parse()
                .map_err(|e| format!("Failed to parse test `{}`: {:?}", line_test, e))?,
            target_true: line_true
                .split("If true: throw to monkey ")
                .nth(1)
                .ok_or(format!("Invalid test true line: {}", line_true))?
                .parse()
                .map_err(|e| format!("Failed to parse target true `{}`: {:?}", line_true, e))?,
            target_false: line_false
                .split("If false: throw to monkey ")
                .nth(1)
                .ok_or(format!("Invalid test false line: {}", line_false))?
                .parse()
                .map_err(|e| format!("Failed to parse target false `{}`: {:?}", line_false, e))?,
            n_inspected: 0,
        })
    }
}

fn parse_monkeys(s: &str) -> Result<Vec<Monkey>, Box<dyn Error>> {
    s.split("\n\n").map(Monkey::from_str).collect()
}

fn monkey_round<F>(monkeys: &mut Vec<Monkey>, worry_update: F)
where
    F: Fn(usize) -> usize,
{
    for i in 0..monkeys.len() {
        while !monkeys[i].items.is_empty() {
            let mut item = monkeys[i].items.pop_front().unwrap();
            let monkey = &mut monkeys[i];

            item = match monkey.op {
                Op::Add(n) => item + n,
                Op::Mul(n) => item * n,
                Op::Square => item * item,
            };

            item = worry_update(item);
            let target = if item % monkey.test_div == 0 {
                monkey.target_true
            } else {
                monkey.target_false
            };

            monkey.n_inspected += 1;
            monkeys[target].items.push_back(item);
        }
    }
}

fn top2(mut it: impl Iterator<Item = usize>) -> [usize; 2] {
    let mut top0 = it.next().unwrap();
    let mut top1 = it.next().unwrap();

    for i in it {
        if i >= top0 {
            (top0, top1) = (i, top0);
        } else if i > top1 {
            top1 = i;
        }
    }

    [top0, top1]
}

fn calc_monkey_business_level(monkeys: &[Monkey]) -> usize {
    top2(monkeys.iter().map(|m| m.n_inspected)).iter().product()
}

fn simulate_monkeys<F>(mut monkeys: Vec<Monkey>, rounds: usize, worry_update: F) -> usize
where
    F: Fn(usize) -> usize,
{
    for _ in 0..rounds {
        monkey_round(&mut monkeys, &worry_update);
    }
    calc_monkey_business_level(&monkeys)
}

fn main() -> Result<(), Box<dyn Error>> {
    let monkeys = parse_monkeys(&fs::read_to_string("input.txt")?)?;
    let modulo: usize = monkeys.iter().map(|m| m.test_div).product();

    println!("A: {}", simulate_monkeys(monkeys.clone(), 20, |w| w / 3));
    println!("B: {}", simulate_monkeys(monkeys, 10000, |w| w % modulo));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn test_a() {
        let monkeys = parse_monkeys(EXAMPLE).unwrap();
        assert_eq!(simulate_monkeys(monkeys, 20, |w| w / 3), 10605);
    }

    #[test]
    fn test_b() {
        let monkeys = parse_monkeys(EXAMPLE).unwrap();
        let modulo: usize = monkeys.iter().map(|m| m.test_div).product();
        assert_eq!(
            simulate_monkeys(monkeys, 10000, |w| w % modulo),
            2_713_310_158
        );
    }
}
