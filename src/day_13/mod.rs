use itertools::Itertools;
use std::collections::BTreeSet;

#[derive(Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

impl From<&str> for Fold {
    fn from(s: &str) -> Self {
        let s = s.split_whitespace().next_back().unwrap();
        match s.chars().nth(0).unwrap() {
            'x' => Self::X(s[2..].parse().unwrap()),
            'y' => Self::Y(s[2..].parse().unwrap()),
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
pub struct Transparency {
    points: BTreeSet<(usize, usize)>,
    folds: Vec<Fold>,
}

impl Transparency {
    fn fold(&mut self) -> &Self {
        match self.folds.pop() {
            Some(Fold::X(x)) => {
                self.points = self
                    .points
                    .iter()
                    .map(|&p| if p.0 > x { (x - (p.0 - x), p.1) } else { p })
                    .collect();
            }
            Some(Fold::Y(y)) => {
                self.points = self
                    .points
                    .iter()
                    .map(|&p| if p.1 > y { (p.0, y - (p.1 - y)) } else { p })
                    .collect();
            }
            None => {}
        }
        self
    }
}

#[aoc_generator(day13)]
pub fn get_input(input: &str) -> Transparency {
    let (points_str, folds_str) = input.split_once("\n\n").unwrap();
    let points: BTreeSet<(usize, usize)> = points_str
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let folds = folds_str.lines().map(|l| l.into()).rev().collect();
    Transparency {
        points: points,
        folds: folds,
    }
}

#[aoc(day13, part1)]
pub fn part_1(transparency: &Transparency) -> usize {
    let mut transparency = transparency.clone();
    transparency.fold();
    transparency.points.len()
}

#[aoc(day13, part2)]
pub fn part_2(transparency: &Transparency) -> String {
    let mut transparency = transparency.clone();
    while !transparency.folds.is_empty() {
        transparency.fold();
    }
    let x_max = transparency.points.iter().map(|p| p.0).max().unwrap() + 1;
    let y_max = transparency.points.iter().map(|p| p.1).max().unwrap() + 1;
    let mut printout = vec![vec![" "; x_max]; y_max];
    for p in transparency.points {
        printout[p.1][p.0] = "█";
    }
    "\n".to_owned() + printout.iter().map(|l| l.join("")).join("\n").as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../../test_data/day_13.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 17);
    }

    #[test]
    fn test_part_2() {}
}
