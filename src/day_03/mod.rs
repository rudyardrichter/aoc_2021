use bitvec::prelude::*;
use std::collections::HashSet;

#[aoc_generator(day3)]
pub fn get_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(2).unwrap() as usize).collect())
        .collect()
}

fn gamma_vec(numbers: &Vec<Vec<usize>>) -> BitVec {
    let n: usize = numbers.len();
    numbers
        .iter()
        // why isn't reduce working here?
        .fold(vec![0; n], |s, v| {
            s.into_iter().zip(v).map(|(a, b)| a + b).collect()
        })
        .iter()
        .map(|&count| count >= n / 2)
        .collect()
}

fn epsilon_from_gamma(gamma: &BitVec) -> BitVec {
    !gamma.clone()
}

fn bitvec_to_usize(bits: &BitVec) -> usize {
    bits.iter().fold(0, |sum, bit| (sum << 1) + (*bit as usize))
}

#[aoc(day3, part1)]
pub fn part_1(numbers: &Vec<Vec<usize>>) -> usize {
    let gamma = gamma_vec(numbers);
    let epsilon = epsilon_from_gamma(&gamma);
    bitvec_to_usize(&gamma) * bitvec_to_usize(&epsilon)
}

fn life_support(numbers: &Vec<Vec<usize>>, compare: fn(usize, usize) -> bool) -> usize {
    let mut remaining: HashSet<usize> = (0..numbers.len()).collect();
    let mut i: usize = 0;
    while remaining.len() > 1 && i < numbers.len() {
        let mut zeroes: HashSet<usize> = HashSet::new();
        let mut ones: HashSet<usize> = HashSet::new();
        for &r in remaining.iter() {
            if numbers[r][i] == 0 {
                zeroes.insert(r);
            } else {
                ones.insert(r);
            }
        }
        if compare(zeroes.len(), ones.len()) {
            for x in zeroes.iter() {
                remaining.remove(x);
            }
        } else {
            for x in ones.iter() {
                remaining.remove(x);
            }
        }
        i += 1;
    }
    bitvec_to_usize(
        &numbers[*remaining.iter().next().unwrap()]
            .iter()
            .map(|&x| x > 0)
            .collect(),
    )
}

#[aoc(day3, part2)]
pub fn part_2(numbers: &Vec<Vec<usize>>) -> usize {
    let o2 = life_support(&numbers, |n_zeroes, n_ones| n_zeroes <= n_ones);
    let co2 = life_support(&numbers, |n_zeroes, n_ones| n_zeroes > n_ones);
    o2 * co2
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";

    #[test]
    fn test_part_1() {
        assert_eq!(bitvec_to_usize(&gamma_vec(&get_input(INPUT))), 22);
        assert_eq!(part_1(&get_input(INPUT)), 198);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 230);
    }
}
