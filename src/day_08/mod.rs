pub struct Entry {
    signal: [Vec<char>; 10],
    output: [Vec<char>; 4],
}

impl Entry {
    fn chars_for_one(&self) -> [char; 2] {
        self.signal
            .clone()
            .into_iter()
            .find(|s| s.len() == 2)
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn chars_for_four(&self) -> [char; 4] {
        self.signal
            .clone()
            .into_iter()
            .find(|s| s.len() == 4)
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn overlap_one(&self, signal: &Vec<char>) -> usize {
        signal
            .iter()
            .filter(|c| self.chars_for_one().contains(c))
            .count()
    }

    fn overlap_four(&self, signal: &Vec<char>) -> usize {
        signal
            .iter()
            .filter(|c| self.chars_for_four().contains(c))
            .count()
    }

    fn value(&self) -> usize {
        // Of the digits we can't identify immediately (0, 2, 3, 5, 6, and 9), the following table
        // identifies them based on their number of segments S, and the number of segments where
        // that digit overlaps with a digit we can immediately identify (1, 4, 7, or 8; except
        // overlap with 8 is just the number of digits).
        //
        // digit | S | 1 | 4 | 7 |
        // ------+---+---+---+---+
        //     0 | 6 | 2 | 3 | 3 |
        //     2 | 5 | 1 | 2 | 2 |
        //     3 | 5 | 2 | 3 | 3 |
        //     5 | 5 | 1 | 3 | 2 |
        //     6 | 6 | 1 | 3 | 2 |
        //     9 | 6 | 2 | 4 | 3 |
        //
        //  No combination of 2 properties is enough to uniquely identify them; nor 1, 4, 7
        //  properties; nor S, 1, 7. S, 4, 7 or S, 1, 4 are both enough.
        self.output
            .iter()
            .map(|o| match o.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                n => match (n, self.overlap_one(o), self.overlap_four(o)) {
                    (6, 2, 3) => 0,
                    (5, 1, 2) => 2,
                    (5, 2, 3) => 3,
                    (5, 1, 3) => 5,
                    (6, 1, 3) => 6,
                    _ => 9,
                },
            })
            .reduce(|sum, n| sum * 10 + n)
            .unwrap()
    }
}

impl From<&str> for Entry {
    fn from(string: &str) -> Self {
        let (s_signal, s_output) = string.split_once("|").unwrap();
        Self {
            signal: s_signal
                .split_whitespace()
                .map(|s| s.chars().collect())
                .collect::<Vec<Vec<char>>>()
                .try_into()
                .unwrap(),
            output: s_output
                .split_whitespace()
                .map(|s| s.chars().collect())
                .collect::<Vec<Vec<char>>>()
                .try_into()
                .unwrap(),
        }
    }
}

#[aoc_generator(day8)]
pub fn get_input(input: &str) -> Vec<Entry> {
    input.lines().map(Entry::from).collect()
}

#[aoc(day8, part1)]
pub fn part_1(entries: &Vec<Entry>) -> usize {
    entries
        .iter()
        .map(|e| {
            e.output
                .iter()
                .filter(|o| match o.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
pub fn part_2(entries: &Vec<Entry>) -> usize {
    entries.iter().map(|e| e.value()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../../test_data/day_08.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 61229);
    }
}
