#[derive(Debug)]
pub enum Bracket {
    Open(char),
    Close(char),
}

enum Error {
    Corrupted(usize),
    Incomplete(usize),
}

impl Into<usize> for Error {
    fn into(self) -> usize {
        match self {
            Self::Corrupted(n) => n,
            Self::Incomplete(n) => n,
        }
    }
}

impl Bracket {
    fn score_corrupted(&self) -> usize {
        let score = |&bracket| match bracket {
            '(' => 3,
            '[' => 57,
            '{' => 1197,
            '<' => 25137,
            _ => panic!("wrong bracket char"),
        };
        match self {
            Self::Open(bracket) => score(bracket),
            Self::Close(bracket) => score(bracket),
        }
    }

    fn score_incomplete(&self) -> usize {
        let score = |&bracket| match bracket {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("wrong bracket char"),
        };
        match self {
            Self::Open(bracket) => score(bracket),
            Self::Close(bracket) => score(bracket),
        }
    }
}

impl TryFrom<char> for Bracket {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Bracket::Open('(')),
            '[' => Ok(Bracket::Open('[')),
            '{' => Ok(Bracket::Open('{')),
            '<' => Ok(Bracket::Open('<')),
            ')' => Ok(Bracket::Close('(')),
            ']' => Ok(Bracket::Close('[')),
            '}' => Ok(Bracket::Close('{')),
            '>' => Ok(Bracket::Close('<')),
            _ => Err(format!("not a valid bracket: {}", c)),
        }
    }
}

fn error_score(brackets: &Vec<Bracket>) -> Error {
    let mut stack: Vec<&Bracket> = Vec::new();
    for b in brackets {
        match b {
            Bracket::Open(_) => {
                stack.push(b);
            }
            Bracket::Close(close) => {
                if let Some(Bracket::Open(open)) = stack.pop() {
                    if open != close {
                        return Error::Corrupted(b.score_corrupted());
                    }
                } else {
                    return Error::Corrupted(b.score_corrupted());
                }
            }
        }
    }
    Error::Incomplete(
        stack
            .iter()
            .rev()
            .fold(0, |sum, b| 5 * sum + b.score_incomplete()),
    )
}

#[aoc_generator(day10)]
pub fn get_input(input: &str) -> Vec<Vec<Bracket>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.try_into().unwrap()).collect())
        .collect()
}

#[aoc(day10, part1)]
pub fn part_1(bracket_lines: &Vec<Vec<Bracket>>) -> usize {
    bracket_lines
        .iter()
        .filter_map(|l| match error_score(l) {
            Error::Corrupted(score) => Some(score),
            _ => None,
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn part_2(bracket_lines: &Vec<Vec<Bracket>>) -> usize {
    let mut scores: Vec<usize> = bracket_lines
        .iter()
        .filter_map(|l| match error_score(l) {
            Error::Incomplete(score) => Some(score),
            _ => None,
        })
        .collect();
    scores.sort();
    scores[scores.len().div_euclid(2)]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../../test_data/day_10.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 26397);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 288957);
    }
}
