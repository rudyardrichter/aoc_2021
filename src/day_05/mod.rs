use geo::{Coordinate, Line};
use num::traits::Zero;
use std::cmp::Ordering;
use std::collections::HashMap;

fn parse_tuple(t: &str) -> (isize, isize) {
    let (t_a, t_b) = t.split_once(",").unwrap();
    (t_a.parse().unwrap(), t_b.parse().unwrap())
}

#[aoc_generator(day5)]
pub fn get_input(input: &str) -> Vec<Line<isize>> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(" -> ").unwrap();
            let c1: Coordinate<isize> = parse_tuple(a).into();
            let c2: Coordinate<isize> = parse_tuple(b).into();
            match c1.x.cmp(&c2.x) {
                Ordering::Less => Line::new(c1, c2),
                Ordering::Equal => match c1.y.cmp(&c2.y) {
                    Ordering::Less => Line::new(c1, c2),
                    _ => Line::new(c2, c1),
                },
                Ordering::Greater => Line::new(c2, c1),
            }
        })
        .collect()
}

fn update_vents(vents: &mut HashMap<(isize, isize), usize>, line: &Line<isize>) -> () {
    if line.dx() == 0 {
        let y_min = std::cmp::min(line.start.y, line.end.y);
        let y_max = std::cmp::max(line.start.y, line.end.y);
        for xy in std::iter::repeat(line.start.x).zip(y_min..=y_max) {
            *vents.entry(xy).or_insert(0) += 1;
        }
    } else if line.dy() == 0 {
        let x_min = std::cmp::min(line.start.x, line.end.x);
        let x_max = std::cmp::max(line.start.x, line.end.x);
        for xy in (x_min..=x_max).zip(std::iter::repeat(line.start.y)) {
            *vents.entry(xy).or_insert(0) += 1;
        }
    } else if line.dx().abs() == line.dy().abs() {
        let x_min = std::cmp::min(line.start.x, line.end.x);
        let x_max = std::cmp::max(line.start.x, line.end.x);
        let y_min = std::cmp::min(line.start.y, line.end.y);
        let y_max = std::cmp::max(line.start.y, line.end.y);
        if line.start.y < line.end.y {
            for xy in (x_min..=x_max).zip(y_min..=y_max) {
                *vents.entry(xy).or_insert(0) += 1;
            }
        } else {
            for xy in (x_min..=x_max).zip((y_min..=y_max).rev()) {
                *vents.entry(xy).or_insert(0) += 1;
            }
        }
    }
}

#[aoc(day5, part1)]
pub fn part_1(lines: &Vec<Line<isize>>) -> usize {
    let mut vents: HashMap<(isize, isize), usize> = HashMap::new();
    lines
        .iter()
        .filter(|l| l.dx().is_zero() || l.dy().is_zero())
        .for_each(|l| update_vents(&mut vents, l));
    vents.values().filter(|v| **v >= 2).count()
}

#[aoc(day5, part2)]
pub fn part_2(lines: &Vec<Line<isize>>) -> usize {
    let mut vents: HashMap<(isize, isize), usize> = HashMap::new();
    lines
        .iter()
        .filter(|l| l.dx().is_zero() || l.dy().is_zero() || l.dx().abs() == l.dy().abs())
        .for_each(|l| update_vents(&mut vents, l));
    vents.values().filter(|v| **v >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_file() -> &'static str {
        include_str!("../../test_data/day_05.txt")
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(test_file())), 5);
        assert_eq!(part_1(&get_input("0,0 -> 0,4\n0,0 -> 0,4")), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(test_file())), 12);
    }
}
