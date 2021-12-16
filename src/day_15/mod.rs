use pathfinding::directed::astar::astar;

pub struct Graph {
    costs: Vec<usize>,
    w: usize,
}

impl Graph {
    fn neighbors(&self, i: usize) -> impl std::iter::IntoIterator<Item = usize> {
        let mut result: Vec<usize> = Vec::new();
        let x = i % self.w;
        if i >= self.w {
            result.push(i - self.w); // ↑
        }
        if x > 0 {
            result.push(i - 1); // ←
        }
        if x < self.w - 1 {
            result.push(i + 1); // →
        }
        if i < self.costs.len() - self.w {
            result.push(i + self.w); // ↓
        }
        result.into_iter()
    }

    fn risk(&self) -> usize {
        astar(
            &0,
            |&i| {
                self.neighbors(i)
                    .into_iter()
                    .map(|j| (j, self.costs[j]))
                    .collect::<Vec<_>>()
            },
            |&i| (self.w - 1 - i % self.w) + (self.costs.len() / self.w - i / self.w),
            |&i| i == self.costs.len() - 1,
        )
        .unwrap()
        .1
    }
}

#[aoc_generator(day15)]
pub fn get_input(input: &str) -> Graph {
    let lines = input.lines().map(|l| {
        l.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    });
    Graph {
        costs: lines.clone().flatten().collect(),
        w: lines.collect::<Vec<Vec<usize>>>()[0].len(),
    }
}

#[aoc(day15, part1)]
pub fn part_1(graph: &Graph) -> usize {
    graph.risk()
}

fn wrap(i: usize, m: usize) -> usize {
    if i >= m {
        i % m + 1
    } else {
        i
    }
}

#[aoc(day15, part2)]
pub fn part_2(graph: &Graph) -> usize {
    let mut costs = vec![0; graph.costs.len() * 5 * 5];
    let w_5 = graph.w * 5;
    for i in 0..costs.len() {
        let (row_5, col_5) = (i / w_5, i % w_5);
        let row_1 = row_5 % (graph.costs.len() / graph.w);
        let col_1 = col_5 % graph.w;
        let (dx, dy) = (col_5 / (graph.costs.len() / graph.w), row_5 / graph.w);
        costs[i] = wrap(graph.costs[graph.w * row_1 + col_1] + dx + dy, 10);
    }
    let graph = Graph {
        costs: costs,
        w: w_5,
    };
    graph.risk()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../../test_data/day_15.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input("1163\n1381\n2136\n3694\n")), 17);
        assert_eq!(part_1(&get_input(INPUT)), 40);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 315);
    }
}
