#[aoc_generator(day6)]
pub fn get_input(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn fish_after_days<'a>(fish: &'a Vec<usize>) -> impl FnOnce(usize) -> usize {
    let fish = fish.clone();
    move |days| {
        let mut buckets: [usize; 9] = [0; 9];
        for f in fish {
            buckets[f] += 1;
        }
        for _ in 0..days {
            let fish_zero = buckets[0];
            for i in 1..buckets.len() {
                buckets[i - 1] = buckets[i];
            }
            buckets[6] += fish_zero;
            buckets[8] = fish_zero;
        }
        buckets.iter().sum()
    }
}

#[aoc(day6, part1)]
pub fn part_1(fish: &Vec<usize>) -> usize {
    fish_after_days(fish)(80)
}

#[aoc(day6, part2)]
pub fn part_2(fish: &Vec<usize>) -> usize {
    fish_after_days(fish)(256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let fish = &get_input("3,4,3,1,2");
        assert_eq!(fish_after_days(fish)(18), 26);
        assert_eq!(part_1(&fish), 5934);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input("3,4,3,1,2")), 26984457539);
    }
}
