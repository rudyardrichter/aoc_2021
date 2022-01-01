use itertools::Itertools;
use std::collections::VecDeque;

const MAX_DEPTH: usize = 4;

#[derive(Clone, Copy, Debug)]
struct Node {
    d: usize,
    x: usize,
}

#[derive(Clone)]
pub struct Snailfish {
    numbers: Vec<Node>,
}

impl Snailfish {
    fn reduce(&mut self) -> () {
        while self.reduce_step() {}
    }

    fn reduce_step(&mut self) -> bool {
        let mut changed = false;
        let mut iter = self.numbers.iter();
        let mut before: Vec<Node> = iter
            .peeking_take_while(|number| number.d <= MAX_DEPTH)
            .cloned()
            .collect();
        if let Some(exploding_left) = iter.next() {
            if let Some(next_left) = before.last_mut() {
                next_left.x += exploding_left.x;
            }
            before.push(Node {
                d: exploding_left.d - 1,
                x: 0,
            });
            changed = true;
            if let Some(exploding_right) = iter.next() {
                if let Some(next_right) = iter.next() {
                    before.push(Node {
                        d: next_right.d,
                        x: next_right.x + exploding_right.x,
                    });
                }
            }
            before.extend(iter);
            self.numbers = before;
        } else {
            let mut iter = self.numbers.iter();
            let mut before: Vec<Node> = iter
                .peeking_take_while(|number| number.x < 10)
                .cloned()
                .collect();
            if let Some(&to_split) = iter.next() {
                let q = to_split.x / 2;
                before.push(Node {
                    d: to_split.d + 1,
                    x: q,
                });
                before.push(Node {
                    d: to_split.d + 1,
                    x: to_split.x - q,
                });
                changed = true;
            }
            before.extend(iter);
            self.numbers = before;
        }
        changed
    }

    fn magnitude(&self) -> usize {
        let mut numbers: VecDeque<Node> = self.numbers.clone().into();
        for d in (1..=MAX_DEPTH).rev() {
            let mut keep = VecDeque::new();
            while let Some(n) = numbers.pop_front() {
                if n.d == d {
                    let next = numbers.pop_front().unwrap();
                    keep.push_back(Node {
                        d: d - 1,
                        x: 3 * n.x + 2 * next.x,
                    });
                } else {
                    keep.push_back(n);
                }
            }
            numbers = keep;
        }
        numbers.pop_front().unwrap().x
    }
}

impl TryFrom<&str> for Snailfish {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut numbers = Vec::new();
        let mut d: usize = 0;
        for c in s.chars() {
            match c {
                '[' => d += 1,
                ']' => {
                    d = d.checked_sub(1).ok_or("imbalanced brackets")?;
                }
                ',' => {}
                c => match c.to_digit(10) {
                    Some(x) => {
                        numbers.push(Node {
                            d: d,
                            x: x as usize,
                        });
                    }
                    None => return Err(format!("couldn't parse character: {}", c)),
                },
            }
        }
        Ok(Snailfish { numbers: numbers })
    }
}

impl std::ops::Add for Snailfish {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut numbers = self.numbers.clone();
        numbers.extend(other.numbers);
        numbers.iter_mut().for_each(|n| (*n).d += 1);
        let mut result = Self { numbers: numbers };
        result.reduce();
        result
    }
}

#[aoc_generator(day18)]
pub fn get_input(input: &str) -> VecDeque<Snailfish> {
    input.lines().map(|l| l.try_into().unwrap()).collect()
}

#[aoc(day18, part1)]
pub fn part_1(snails: &VecDeque<Snailfish>) -> usize {
    let mut snails = snails.clone();
    let mut result = snails.pop_front().unwrap();
    while let Some(snail) = snails.pop_front() {
        result = result + snail;
    }
    result.magnitude()
}

#[aoc(day18, part2)]
pub fn part_2(snails: &VecDeque<Snailfish>) -> usize {
    (0..snails.len())
        .cartesian_product(0..snails.len())
        .filter(|(i, j)| i != j)
        .map(|(i, j)| (snails[i].clone() + snails[j].clone()).magnitude())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../../test_data/day_18.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input("[[1,2],[[3,4],5]]")), 143);
        assert_eq!(
            part_1(&get_input("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")),
            1384
        );
        assert_eq!(part_1(&get_input("[[[[1,1],[2,2]],[3,3]],[4,4]]")), 445);
        assert_eq!(
            part_1(&get_input("[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]")),
            1384
        );

        assert_eq!(part_1(&get_input(INPUT)), 4140);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 3993);
    }
}
