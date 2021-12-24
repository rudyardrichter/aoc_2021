use sscanf::scanf;

fn triangle_number(n: isize) -> isize {
    (n * n + n).div_euclid(2)
}

fn triangle_index(n: isize) -> isize {
    (0..)
        .skip_while(|&i| triangle_number(i) < n)
        .take(1)
        .next()
        .unwrap()
}

pub struct Target {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Target {
    fn n_possible_vs(&self) -> usize {
        let mut result = 0;
        for v_x in self.v_xs() {
            for v_y in self.possible_v_ys() {
                let (mut x, mut y) = (0, 0);
                for i in 0.. {
                    x += std::cmp::max(v_x - i, 0);
                    y += v_y - i;
                    if self.x_min <= x && x <= self.x_max && self.y_min <= y && y <= self.y_max {
                        result += 1;
                        break;
                    }
                    if x > self.x_max || y < self.y_min {
                        break;
                    }
                }
            }
        }
        result
    }

    fn v_xs(&self) -> impl Iterator<Item = isize> {
        let mut result: Vec<isize> = (self.x_min..=self.x_max).collect();
        let v_x_min = triangle_index(self.x_min);
        let v_x_max = self.x_max.div_euclid(2) + 1;
        result.append(&mut (v_x_min..=v_x_max).collect());
        result.into_iter()
    }

    fn possible_v_ys(&self) -> impl Iterator<Item = isize> {
        -self.y_min.abs()..self.y_min.abs()
    }
}

impl TryFrom<&str> for Target {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parsed = scanf!(
            s,
            "target area: x={}..{}, y={}..{}",
            isize,
            isize,
            isize,
            isize
        )
        .ok_or("parse error")?;
        Ok(Target {
            x_min: parsed.0,
            x_max: parsed.1,
            y_min: parsed.2,
            y_max: parsed.3,
        })
    }
}

#[aoc_generator(day17)]
pub fn get_input(input: &str) -> Target {
    input.try_into().unwrap()
}

#[aoc(day17, part1)]
pub fn part_1(target: &Target) -> isize {
    triangle_number(target.y_min)
}

#[aoc(day17, part2)]
pub fn part_2(target: &Target) -> usize {
    target.n_possible_vs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input("target area: x=20..30, y=-10..-5")), 45);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input("target area: x=20..30, y=-10..-5")), 112);
    }
}
