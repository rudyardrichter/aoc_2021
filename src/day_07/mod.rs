#[aoc_generator(day7)]
pub fn get_input(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn part_1(crabs: &Vec<usize>) -> usize {
    let mut clone = crabs.clone();
    let (_, m, _) = clone.select_nth_unstable(crabs.len().div_euclid(2));
    crabs
        .iter()
        .map(|&crab| m.checked_sub(crab).unwrap_or_else(|| crab - *m))
        .sum()
}

fn triangle_number(n: usize) -> usize {
    (n * n + n).div_euclid(2)
}

#[aoc(day7, part2)]
pub fn part_2(crabs: &Vec<usize>) -> usize {
    let total_fuel_to = |pos: usize| -> usize {
        crabs
            .iter()
            .map(|&crab| triangle_number(pos.checked_sub(crab).unwrap_or_else(|| crab - pos)))
            .sum()
    };
    let m = crabs.iter().sum::<usize>().div_euclid(crabs.len());
    std::cmp::min(total_fuel_to(m), total_fuel_to(m + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input("16,1,2,0,4,2,7,1,2,14")), 37);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input("16,1,2,0,4,2,7,1,2,14")), 168);
    }
}
