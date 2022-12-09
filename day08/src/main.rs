use itertools::iproduct;
use std::cmp;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::ops;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridPos {
    row: isize,
    col: isize,
}

#[derive(Debug, Clone, Copy)]
struct GridDelta {
    drow: isize,
    dcol: isize,
}

struct Grid {
    tree_heights: Vec<Vec<char>>,
    n_rows: usize,
    n_cols: usize,
}

struct GridIter<'a> {
    grid: &'a Grid,
    delta: GridDelta,
    pos: GridPos,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl ops::Add<GridDelta> for GridPos {
    type Output = GridPos;

    fn add(self, rhs: GridDelta) -> Self::Output {
        GridPos {
            row: self.row + rhs.drow,
            col: self.col + rhs.dcol,
        }
    }
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Grid, Self::Err> {
        let lines = input.lines().collect::<Vec<_>>();
        let n_cols = lines[0].len();

        if lines.iter().any(|line| line.len() != n_cols) {
            return Err("Mismatched line lengths");
        }

        Ok(Grid {
            n_rows: lines.len(),
            n_cols: n_cols,
            tree_heights: lines
                .iter()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect(),
        })
    }
}

impl<'a> Iterator for GridIter<'a> {
    type Item = &'a char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.grid.get(self.pos) {
            Some(height) => {
                self.pos = self.pos + self.delta;
                Some(height)
            }
            None => None,
        }
    }
}

impl Grid {
    fn iter_pos(&self, start: GridPos) -> impl Iterator<Item = (GridPos, &char)> {
        iproduct!(
            (start.col as usize)..self.n_cols,
            (start.row as usize)..self.n_rows
        )
        .map(|(i, j)| {
            let pos = GridPos {
                row: i as isize,
                col: j as isize,
            };
            (pos, self.get(pos).unwrap())
        })
    }

    fn iter_direction(&self, start: GridPos, direction: &Direction) -> GridIter<'_> {
        GridIter {
            grid: self,
            delta: match direction {
                Direction::Up => GridDelta { drow: -1, dcol: 0 },
                Direction::Down => GridDelta { drow: 1, dcol: 0 },
                Direction::Left => GridDelta { drow: 0, dcol: -1 },
                Direction::Right => GridDelta { drow: 0, dcol: 1 },
            },
            pos: start,
        }
    }

    fn get(&self, pos: GridPos) -> Option<&char> {
        let row = match usize::try_from(pos.row) {
            Err(_) => return None,
            Ok(i) => i,
        };
        let col = match usize::try_from(pos.col) {
            Err(_) => return None,
            Ok(i) => i,
        };
        match self.tree_heights.get(row) {
            None => None,
            Some(row) => match row.get(col) {
                None => None,
                Some(height) => Some(height),
            },
        }
    }
}

fn count_visible_trees(grid: &Grid) -> usize {
    let mut visible = HashSet::<GridPos>::new();

    for (pos, height) in grid.iter_pos(GridPos { row: 0, col: 0 }) {
        [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ]
        .iter()
        .filter_map(|dir| {
            (grid
                    .iter_direction(pos, dir)
                    .skip(1) // skip current tree
                    .max()
                    .unwrap_or(&'\x00') // handle case where the first tree is already larger
                    < height)
                .then(|| pos)
        })
        .for_each(|pos| {
            visible.insert(pos);
        });
    }
    visible.len()
}

fn calc_max_tree_score(grid: &Grid) -> usize {
    grid.iter_pos(GridPos { row: 1, col: 1 })
        .map(|(pos, height)| {
            [
                (Direction::Left, pos.col as isize),
                (Direction::Right, (grid.n_cols as isize) - pos.col - 1),
                (Direction::Up, pos.row as isize),
                (Direction::Down, (grid.n_rows as isize) - pos.row - 1),
            ]
            .iter()
            .map(|(dir, max)| {
                grid.iter_direction(pos, dir)
                    .enumerate()
                    .skip(1) // skip tree itself which doesn't contribute to the score
                    .filter_map(|(k, k_height)| (k_height >= height).then(|| k))
                    .nth(0) // stop at the first tree >= this one
                    .unwrap_or(cmp::max(*max, 0) as usize) // handle case where all tree are smaller
            })
            .product::<usize>()
        })
        .max()
        .unwrap_or(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("input.txt")?;
    let grid = Grid::from_str(&data)?;
    println!("A: {}", count_visible_trees(&grid));
    println!("B: {}", calc_max_tree_score(&grid));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn test_a() {
        assert_eq!(
            count_visible_trees(&Grid::from_str(&EXAMPLE_DATA).unwrap()),
            21
        );
    }

    #[test]
    fn test_b() {
        assert_eq!(
            calc_max_tree_score(&Grid::from_str(&EXAMPLE_DATA).unwrap()),
            8
        );
    }
}
