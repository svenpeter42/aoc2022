use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn calc_score(&c: &char) -> Result<usize, &'static str> {
    match c as u8 {
        b'A'..=b'Z' => Ok((c as u8 - b'A' + 27) as usize),
        b'a'..=b'z' => Ok((c as u8 - b'a' + 1) as usize),
        _ => Err("invalid item"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let score: usize = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| {
            let l = line.len();
            let left = line[..l / 2].chars().collect::<HashSet<_>>();
            line[l / 2..]
                .chars()
                .find(|&c| left.contains(&c))
                .iter()
                .map(|c| calc_score(c).unwrap())
                .sum::<usize>()
        })
        .sum();
    println!("A: {score}");

    let score2: usize = fs::read_to_string("input.txt")?
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(x, y, z)| {
            let xs = x.chars().collect::<HashSet<_>>();
            let ys = y.chars().collect::<HashSet<_>>();
            let zs = z.chars().collect::<HashSet<_>>();
            let xy: HashSet<_> = xs.intersection(&ys).copied().collect();
            let xyz: HashSet<_> = xy.intersection(&zs).collect();
            assert_eq!(1, xyz.len());
            xyz.iter().map(|c| calc_score(c).unwrap()).sum::<usize>()
        })
        .sum();
    println!("B: {score2}");
    Ok(())
}
