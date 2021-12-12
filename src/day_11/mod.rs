use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug)]
pub struct Octopi {
    // flattened data in reading order
    data: Vec<u8>,
    // row width
    w: usize,
}

impl Octopi {
    fn new<T: Into<u8>>(data: Vec<T>, w: usize) -> Self {
        Octopi {
            data: data.into_iter().map(T::into).collect(),
            w: w,
        }
    }

    fn neighbors(&self, i: usize) -> impl std::iter::IntoIterator<Item = usize> {
        let mut result: Vec<usize> = Vec::new();
        let x = i % self.w;
        if i >= self.w && x > 0 {
            result.push(i - self.w - 1); // ↖
        }
        if i >= self.w {
            result.push(i - self.w); // ↑
        }
        if i >= self.w && i % self.w < 9 {
            result.push(i - self.w + 1); // ↗
        }
        if x > 0 {
            result.push(i - 1); // ←
        }
        if i % self.w < 9 {
            result.push(i + 1); // →
        }
        if i < self.data.len() - self.w && i % self.w > 0 {
            result.push(i + self.w - 1) // ↙
        }
        if i < self.data.len() - self.w {
            result.push(i + self.w); // ↓
        }
        if i < self.data.len() - self.w && i % self.w < 9 {
            result.push(i + self.w + 1); // ↘
        }
        result.into_iter()
    }

    fn inc(&mut self, i: usize) -> bool {
        if let Some(n) = self.data[i].checked_add(1) {
            self.data[i] = n;
        }
        self.data[i] > 9
    }

    fn step(&mut self) -> usize {
        let mut flashed = VecDeque::from_iter((0..self.data.len()).filter(|&i| self.inc(i)));
        let mut already_flashed: HashSet<usize> = HashSet::from_iter(flashed.iter().cloned());
        while let Some(i) = flashed.pop_front() {
            for j in self.neighbors(i) {
                if !already_flashed.contains(&j) {
                    if self.inc(j) {
                        flashed.push_back(j);
                        already_flashed.insert(j);
                    }
                }
            }
        }
        for i in 0..self.data.len() {
            if self.data[i] > 9 {
                self.data[i] = 0
            }
        }
        already_flashed.len()
    }
}

impl std::fmt::Display for Octopi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .as_slice()
                .chunks(self.w)
                .into_iter()
                .map(|line| line
                    .into_iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(""))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[aoc_generator(day11)]
pub fn get_input(input: &str) -> Octopi {
    let lines: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|d| d.to_digit(10).unwrap() as u8).collect())
        .collect();
    let w: usize = lines[0].len();
    Octopi::new(lines.into_iter().flatten().collect(), w)
}

#[aoc(day11, part1)]
pub fn part_1(octopi: &Octopi) -> usize {
    let mut octopi = octopi.clone();
    (0..100).fold(0, |sum, _| sum + octopi.step())
}

#[aoc(day11, part2)]
pub fn part_2(octopi: &Octopi) -> usize {
    let mut octopi = octopi.clone();
    let n = octopi.data.len();
    let mut i: usize = 1;
    while octopi.step() != n {
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../../test_data/day_11.txt");

    #[test]
    fn test_part_1() {
        let mut octopi = get_input("11111\n19991\n19191\n19991\n11111");
        assert_eq!((0..2).fold(0, |sum, _| sum + octopi.step()), 9);
        assert_eq!(part_1(&get_input(INPUT)), 1656);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 195);
    }
}
