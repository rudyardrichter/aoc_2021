use itertools::{Itertools, TupleWindows};
use std::iter::Map;

#[aoc_generator(day1)]
pub fn get_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

fn count_increases<I: Clone + IntoIterator<Item = usize>>(depths: &I) -> usize {
    depths
        .clone()
        .into_iter()
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

#[aoc(day1, part1)]
pub fn part_1(depths: &Vec<usize>) -> usize {
    count_increases(depths)
}

fn measurement_sum<
    I: Iterator<Item = usize>,
    II: Clone + IntoIterator<Item = usize, IntoIter = I>,
>(
    depths: &II,
) -> Map<TupleWindows<I, (usize, usize, usize)>, fn((usize, usize, usize)) -> usize> {
    depths
        .clone()
        .into_iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
}

#[aoc(day1, part2)]
pub fn part_2(depths: &Vec<usize>) -> usize {
    count_increases(&measurement_sum(depths))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part_1(&input), 7);
    }

    #[test]
    fn test_part_2() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part_2(&input), 5);
    }
}
