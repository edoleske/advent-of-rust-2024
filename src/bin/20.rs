use std::collections::{BinaryHeap, HashMap, HashSet};

advent_of_code::solution!(20);

#[derive(Debug, Hash, PartialEq, Eq, Ord, Copy, Clone, PartialOrd)]
struct Index(i32, i32);

impl Index {
    fn distance(&self, other: &Index) -> u32 {
        (self.0 - other.0).abs() as u32 + (self.1 - other.1).abs() as u32
    }
}

struct Graph {
    nodes: HashSet<Index>,
    edges: HashMap<Index, Vec<Index>>,
    cheats: Vec<(Index, Index)>,
    start: Index,
    end: Index,
}

impl Graph {
    fn new(input: &str) -> Self {
        let array: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();

        let mut nodes = HashSet::new();
        let mut edges = HashMap::new();
        let mut cheats = Vec::new();
        let mut start = Index(0, 0);
        let mut end = Index(0, 0);

        for (y, line) in array.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '#' {
                    nodes.insert(Index(x as i32, y as i32));
                    match c {
                        'S' => start = Index(x as i32, y as i32),
                        'E' => end = Index(x as i32, y as i32),
                        _ => {}
                    }
                }
            }
        }

        // Calculate all edges
        for index in &nodes {
            let mut adjacent = Vec::new();

            for adj in vec![
                Index(index.0 - 1, index.1),
                Index(index.0 + 1, index.1),
                Index(index.0, index.1 - 1),
                Index(index.0, index.1 + 1),
            ] {
                if nodes.contains(&adj) {
                    adjacent.push(adj);
                }
            }

            edges.insert(*index, adjacent);
        }

        // Calculate all possible cheats
        for index in &nodes {
            if index == &end {
                continue;
            }
            let x = index.0;
            let y = index.1;

            if !nodes.contains(&Index(x + 1, y)) && nodes.contains(&Index(x + 2, y)) {
                cheats.push((*index, Index(x + 2, y)));
            }
            if !nodes.contains(&Index(x - 1, y)) && nodes.contains(&Index(x - 2, y)) {
                cheats.push((*index, Index(x - 2, y)));
            }
            if !nodes.contains(&Index(x, y + 1)) && nodes.contains(&Index(x, y + 2)) {
                cheats.push((*index, Index(x, y + 2)));
            }
            if !nodes.contains(&Index(x, y - 1)) && nodes.contains(&Index(x, y - 2)) {
                cheats.push((*index, Index(x, y - 2)));
            }
        }

        Self {
            nodes,
            edges,
            cheats,
            start,
            end,
        }
    }

    fn dijkstra(&self) -> HashMap<Index, u32> {
        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();
        let mut distances = HashMap::new();

        distances.insert(self.start, 0u32);
        queue.push(self.start);

        while let Some(node) = queue.pop() {
            if visited.contains(&node) {
                continue;
            }

            if node == self.end {
                break;
            }

            for edge in self.edges.get(&node).unwrap_or(&Vec::new()).iter() {
                let distance = distances[&node] + 1;

                if let Some(&prev) = distances.get(&edge) {
                    if distance < prev {
                        distances.insert(*edge, distance);
                        queue.push(*edge);
                    }
                } else {
                    distances.insert(*edge, distance);
                    queue.push(*edge);
                }
            }

            visited.insert(node);
        }

        distances
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    let graph = Graph::new(&input);
    let distances = graph.dijkstra();

    let threshold = if input.lines().count() > 15 { 100 } else { 1 };

    for cheat in graph.cheats.iter() {
        let d_from = distances.get(&cheat.0).unwrap();
        let d_to = distances.get(&cheat.1).unwrap();

        if d_from > d_to {
            continue;
        }

        let time_saved = d_to - d_from - 2;
        if time_saved >= threshold {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    let graph = Graph::new(&input);
    let distances = graph.dijkstra();

    let threshold = if input.lines().count() > 15 { 100 } else { 50 };

    for node in graph.nodes.iter() {
        if node == &graph.end {
            continue;
        }

        let d_from = distances.get(&node).unwrap();
        for other in graph.nodes.iter() {
            let d = node.distance(other);
            if d > 20 {
                continue;
            }

            let d_to = distances.get(&other).unwrap();
            if d_from > d_to {
                continue;
            }

            let time_saved = d_to - d_from - d;
            if time_saved >= threshold {
                result += 1;
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}
