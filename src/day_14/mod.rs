use std::collections::HashMap;

#[derive(Clone)]
pub struct Polymer {
    rules: HashMap<(u8, u8), u8>,
    pairs: HashMap<(u8, u8), usize>,
    first: u8,
    last: u8,
}

impl Polymer {
    fn step(&mut self) -> () {
        let mut new_pairs = HashMap::new();
        for (&(left, right), n) in self.pairs.iter() {
            let middle = self.rules[&(left, right)];
            *new_pairs.entry((left, middle)).or_insert(0) += n;
            *new_pairs.entry((middle, right)).or_insert(0) += n;
        }
        self.pairs = new_pairs;
    }

    fn result(&self) -> usize {
        let mut counts: HashMap<u8, usize> = HashMap::new();
        for (&(left, right), n) in self.pairs.iter() {
            *counts.entry(left).or_insert(0) += n;
            *counts.entry(right).or_insert(0) += n;
        }
        let max = counts.iter().max_by_key(|(_, n)| *n).unwrap();
        let (a, mut n_a) = (*max.0, max.1.div_euclid(2));
        let min = counts.iter().min_by_key(|(_, n)| *n).unwrap();
        let (b, mut n_b) = (*min.0, min.1.div_euclid(2));
        (a == self.first).then(|| n_a += 1);
        (a == self.last).then(|| n_a += 1);
        (b == self.first).then(|| n_b += 1);
        (b == self.last).then(|| n_b += 1);
        n_a - n_b
    }
}

impl From<&str> for Polymer {
    fn from(s: &str) -> Self {
        let (template_str, rules_str) = s.split_once("\n\n").unwrap();
        let rules = rules_str
            .lines()
            .map(|l| {
                let (l, r) = l.split_once(" -> ").unwrap();
                ((l.as_bytes()[0], l.as_bytes()[1]), r.as_bytes()[0])
            })
            .collect();
        let pairs = template_str
            .as_bytes()
            .windows(2)
            .fold(HashMap::new(), |mut acc, w| {
                *acc.entry((w[0], w[1])).or_insert(0) += 1;
                acc
            });
        Polymer {
            rules: rules,
            pairs: pairs,
            first: *template_str.as_bytes().first().unwrap(),
            last: *template_str.as_bytes().last().unwrap(),
        }
    }
}

#[aoc_generator(day14)]
pub fn get_input(input: &str) -> Polymer {
    input.into()
}

#[aoc(day14, part1)]
pub fn part_1(polymer: &Polymer) -> usize {
    let mut polymer = polymer.clone();
    (0..10).for_each(|_| polymer.step());
    polymer.result()
}

#[aoc(day14, part2)]
pub fn part_2(polymer: &Polymer) -> usize {
    let mut polymer = polymer.clone();
    (0..40).for_each(|_| polymer.step());
    polymer.result()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../../test_data/day_14.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 1588);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 2188189693529);
    }
}
