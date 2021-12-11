use std::collections::{HashMap, HashSet, VecDeque};

pub struct HeightMap {
    array: Vec<Vec<u8>>,
}

impl HeightMap {
    fn get(&self, x: Option<usize>, y: Option<usize>) -> Option<&u8> {
        match (x, y) {
            (Some(x), Some(y)) => self.array.get(y).map(|r| r.get(x)).flatten(),
            _ => None,
        }
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if x > 0 {
            result.push((x - 1, y));
        }
        if x < self.array[0].len() - 1 {
            result.push((x + 1, y));
        }
        if y > 0 {
            result.push((x, y - 1));
        }
        if y < self.array.len() - 1 {
            result.push((x, y + 1));
        }
        result
    }

    fn neighbor_vals(&self, x: usize, y: usize) -> [Option<&u8>; 4] {
        [
            self.get(x.checked_sub(1), Some(y)),
            self.get(Some(x), y.checked_sub(1)),
            self.get(Some(x), Some(y + 1)),
            self.get(Some(x + 1), Some(y)),
        ]
    }

    fn low_points(&self) -> Vec<(usize, usize)> {
        self.array
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, &n)| {
                        let m: u8 = *self
                            .neighbor_vals(j, i)
                            .iter()
                            .filter_map(|&x| x)
                            .min()
                            .unwrap();
                        if n < m {
                            Some((j, i))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect()
    }
}

#[aoc_generator(day9)]
pub fn get_input(input: &str) -> HeightMap {
    HeightMap {
        array: input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect(),
    }
}

#[aoc(day9, part1)]
pub fn part_1(entries: &HeightMap) -> usize {
    entries
        .low_points()
        .iter()
        .map(|&(x, y)| entries.array[y][x] as usize + 1)
        .sum()
}

#[aoc(day9, part2)]
pub fn part_2(entries: &HeightMap) -> usize {
    let mut basins: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    for low in entries.low_points() {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::from([low]);
        let mut set: HashSet<(usize, usize)> = HashSet::from([low]);
        while let Some(queue_item) = queue.pop_front() {
            for neighbor in entries.neighbors(queue_item).iter() {
                let neighbor_val = entries.array[neighbor.1][neighbor.0];
                let prev_val = entries.array[queue_item.1][queue_item.0];
                if !set.contains(neighbor) && prev_val < neighbor_val && neighbor_val < 9 {
                    queue.push_back(*neighbor);
                    set.insert(*neighbor);
                }
            }
        }
        basins.entry(low).or_insert(set);
    }
    let mut basin_counts: Vec<usize> = basins.values().map(|points| points.len()).collect();
    basin_counts.sort();
    basin_counts.iter().rev().take(3).fold(1, |sum, n| sum * n)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../../test_data/day_09.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 1134);
    }
}
