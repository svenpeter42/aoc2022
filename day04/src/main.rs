#![warn(clippy::pedantic)]

use itertools::Itertools;
use std::error::Error;
use std::fs;

trait Interval<Rhs = Self> {
    fn contains(self, rhs: Rhs) -> bool;
    fn overlaps(self, rhg: Rhs) -> bool;
}

impl Interval for (u32, u32) {
    fn contains(self, other: (u32, u32)) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(self, other: (u32, u32)) -> bool {
        (self.0 <= other.0 && self.1 >= other.0)
            || (self.1 >= other.0 && self.1 <= other.1)
            || (self.0 <= other.1 && self.1 >= other.1)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let score = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| {
            let elf_interval = line
                .split(['-', ','])
                .map(|i| i.parse::<u32>().unwrap())
                .tuples::<(_, _)>()
                .collect_tuple::<((_, _), (_, _))>()
                .unwrap();
            (
                (elf_interval.0.contains(elf_interval.1) || elf_interval.1.contains(elf_interval.0))
                    as u32,
                (elf_interval.0.overlaps(elf_interval.1)) as u32,
            )
        })
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
    println!("{score:?}");
    Ok(())
}
