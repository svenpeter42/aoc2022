use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut data: Vec<u32> = fs::read_to_string("input.txt")?
        .split("\n\n")
        .map(|elf| elf.lines().map(|cals| cals.parse::<u32>().unwrap()).sum())
        .collect();

    data.sort();
    data.reverse();
    let max = data[0];
    let top3 = &data[..3].iter().sum::<u32>();

    println!("A: {max}");
    println!("B: {top3}");
    Ok(())
}
