enum Direction {
    Forward,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "forward" => Self::Forward,
            "up" => Self::Up,
            "down" => Self::Down,
            _ => panic!("couldn't parse direction"),
        }
    }
}

pub struct Move {
    direction: Direction,
    distance: isize,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(" ").collect();
        Move {
            direction: parts[0].into(),
            distance: parts[1].parse::<isize>().unwrap(),
        }
    }
}

#[derive(Clone)]
struct Coordinates {
    x: isize,
    y: isize,
}

impl Coordinates {
    const ZERO: Self = Self { x: 0, y: 0 };

    fn update(&mut self, m: &Move) -> &mut Self {
        match m.direction {
            Direction::Forward => {
                self.x += m.distance;
            }
            Direction::Up => {
                self.y -= m.distance;
            }
            Direction::Down => {
                self.y += m.distance;
            }
        }
        self
    }

    fn multiply(&self) -> isize {
        self.x * self.y
    }
}

#[derive(Clone)]
struct CoordinatesAim {
    x: isize,
    y: isize,
    aim: isize,
}

impl CoordinatesAim {
    const ZERO: Self = Self { x: 0, y: 0, aim: 0 };

    fn update(&mut self, m: &Move) -> &mut Self {
        match m.direction {
            Direction::Forward => {
                self.x += m.distance;
                self.y += self.aim * m.distance;
            }
            Direction::Up => {
                self.aim -= m.distance;
            }
            Direction::Down => {
                self.aim += m.distance;
            }
        }
        self
    }

    fn multiply(&self) -> isize {
        self.x * self.y
    }
}

#[aoc_generator(day2)]
pub fn get_input(input: &str) -> Vec<Move> {
    input.lines().map(|l| l.into()).collect()
}

#[aoc(day2, part1)]
pub fn part_1(moves: &Vec<Move>) -> isize {
    moves
        .iter()
        .fold(&mut Coordinates::ZERO.clone(), |c, m| c.update(m))
        .multiply()
}

#[aoc(day2, part2)]
pub fn part_2(moves: &Vec<Move>) -> isize {
    moves
        .iter()
        .fold(&mut CoordinatesAim::ZERO.clone(), |c, m| c.update(m))
        .multiply()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
        assert_eq!(part_1(&get_input(&input)), 150);
    }

    #[test]
    fn test_part_2() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
        assert_eq!(part_2(&get_input(&input)), 900);
    }
}
