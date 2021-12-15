use std::collections::{HashMap, HashSet};

pub struct Graph {
    nodes: Vec<String>,
    map: HashMap<String, usize>,
    adjacent: HashMap<usize, Vec<usize>>,
    small: HashSet<usize>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            map: HashMap::new(),
            adjacent: HashMap::new(),
            small: HashSet::new(),
        }
    }

    fn index(&self, name: &str) -> Option<usize> {
        self.map.get(name).cloned()
    }

    fn add_node(&mut self, name: &str) -> usize {
        match self.index(name) {
            Some(i) => i,
            None => {
                self.nodes.push(name.to_owned());
                let i = self.nodes.len() - 1;
                self.map.insert(name.to_owned(), i);
                if name.chars().all(|c| matches!(c, 'a'..='z')) {
                    self.small.insert(i);
                }
                i
            }
        }
    }

    fn add_edge(&mut self, name_a: &str, name_b: &str) -> () {
        let i_a = self.index(name_a).unwrap();
        let i_b = self.index(name_b).unwrap();
        self.adjacent.entry(i_a).or_insert(Vec::new()).push(i_b);
        self.adjacent.entry(i_b).or_insert(Vec::new()).push(i_a);
    }
}

#[aoc_generator(day12)]
pub fn get_input<'a>(input: &str) -> Graph {
    let mut graph = Graph::new();
    for line in input.lines() {
        let (src, dst) = line.split_once("-").unwrap();
        graph.add_node(src);
        graph.add_node(dst);
        graph.add_edge(src, dst);
    }
    graph
}

#[aoc(day12, part1)]
pub fn part_1(graph: &Graph) -> usize {
    let start = graph.index("start").unwrap();
    let mut queue: Vec<(usize, HashSet<usize>)> = vec![(start, HashSet::from([start]))];
    let mut paths = 0;
    while let Some((i, visited)) = queue.pop() {
        if i == graph.map["end"] {
            paths += 1;
        } else {
            for &j in graph.adjacent[&i].iter().filter(|j| !visited.contains(j)) {
                if graph.small.contains(&j) {
                    let mut visited_next = visited.clone();
                    visited_next.insert(j);
                    queue.push((j, visited_next));
                } else {
                    queue.push((j, visited.clone()));
                }
            }
        }
    }
    paths
}

#[aoc(day12, part2)]
pub fn part_2(graph: &Graph) -> usize {
    let start = graph.index("start").unwrap();
    let mut queue: Vec<(usize, HashSet<usize>, bool)> =
        vec![(start, HashSet::from([start]), false)];
    let mut paths = 0;
    while let Some((i, visited, revisited)) = queue.pop() {
        if i == graph.map["end"] {
            paths += 1;
        } else {
            for &j in graph.adjacent[&i]
                .iter()
                .filter(|j| **j != graph.map["start"])
            {
                if !visited.contains(&j) {
                    if graph.small.contains(&j) {
                        let mut visited_next = visited.clone();
                        visited_next.insert(j);
                        queue.push((j, visited_next, revisited));
                    } else {
                        queue.push((j, visited.clone(), revisited));
                    }
                } else if !revisited {
                    queue.push((j, visited.clone(), true))
                }
            }
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &'static str = include_str!("../../test_data/day_12_1.txt");
    const INPUT_2: &'static str = include_str!("../../test_data/day_12_2.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT_1)), 10);
        assert_eq!(part_1(&get_input(INPUT_2)), 19);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT_1)), 36);
        assert_eq!(part_2(&get_input(INPUT_2)), 103);
    }
}
