use std::collections::{HashMap, HashSet};
use std::iter;

const BINGO_H: usize = 5;
const BINGO_W: usize = 5;

#[derive(Clone)]
struct Cell {
    n: usize,
    marked: bool,
}

impl Cell {
    fn mark(&mut self) -> &mut Self {
        self.marked = true;
        self
    }
}

impl From<usize> for Cell {
    fn from(n: usize) -> Self {
        Cell {
            n: n,
            marked: false,
        }
    }
}

#[derive(Clone)]
pub struct BingoBoard {
    rows: Vec<Vec<Cell>>,
    lookup: HashMap<usize, (usize, usize)>,
    marks_rows: HashMap<usize, usize>,
    marks_cols: HashMap<usize, usize>,
}

impl BingoBoard {
    fn new(rows: Vec<Vec<Cell>>) -> Self {
        let lookup = rows
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, cell)| (cell.n, (x, y)))
            })
            .flatten()
            .collect();
        Self {
            rows: rows,
            lookup: lookup,
            marks_rows: (0..BINGO_W).zip(iter::repeat(0)).collect(),
            marks_cols: (0..BINGO_H).zip(iter::repeat(0)).collect(),
        }
    }

    fn mark_xy(&mut self, x: usize, y: usize) -> &mut Self {
        self.rows
            .get_mut(y)
            .map(|row| row.get_mut(x).map(Cell::mark));
        self.marks_rows.entry(y).and_modify(|n| *n += 1);
        self.marks_cols.entry(x).and_modify(|n| *n += 1);
        self
    }

    fn mark_value(&mut self, v: usize) -> &mut Self {
        if let Some(&(x, y)) = self.lookup.get(&v) {
            self.mark_xy(x, y);
        }
        self
    }

    fn is_won(&self) -> bool {
        let rows_won = self.marks_rows.values().any(|&v| v >= 5);
        let cols_won = self.marks_cols.values().any(|&v| v >= 5);
        rows_won || cols_won
    }

    fn score(&self) -> usize {
        self.rows
            .iter()
            .map(|row| -> usize {
                row.iter()
                    .filter(|cell| !cell.marked)
                    .map(|cell| cell.n)
                    .sum()
            })
            .sum()
    }
}

impl From<&str> for BingoBoard {
    fn from(s: &str) -> Self {
        BingoBoard::new(
            s.split("\n")
                .map(|row| {
                    row.split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap().into())
                        .collect()
                })
                .collect(),
        )
    }
}

#[aoc_generator(day4)]
pub fn get_input(input: &str) -> (Vec<usize>, Vec<BingoBoard>) {
    let (drawings_line, boards_str) = input.split_once("\n").unwrap();
    let drawings: Vec<usize> = drawings_line
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let boards = boards_str.split("\n\n").map(BingoBoard::from).collect();
    (drawings, boards)
}

#[aoc(day4, part1)]
pub fn part_1((drawings, boards): &(Vec<usize>, Vec<BingoBoard>)) -> usize {
    let mut boards: Vec<BingoBoard> = boards.to_owned();
    for &n in drawings.iter() {
        for b in boards.iter_mut() {
            b.mark_value(n);
            if b.is_won() {
                return n * b.score();
            }
        }
    }
    panic!("no board won");
}

#[aoc(day4, part2)]
pub fn part_2((drawings, boards): &(Vec<usize>, Vec<BingoBoard>)) -> usize {
    let mut boards: Vec<BingoBoard> = boards.to_owned();
    let mut boards_not_won: HashSet<usize> = (0..boards.len()).collect();
    for &n in drawings.iter() {
        for (i, b) in boards.iter_mut().enumerate() {
            b.mark_value(n);
            if b.is_won() {
                boards_not_won.remove(&i);
                if boards_not_won.is_empty() {
                    return n * b.score();
                }
            }
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_file() -> &'static str {
        include_str!("../../test_data/day_04.txt")
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(test_file())), 4512);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(test_file())), 1924);
    }
}
