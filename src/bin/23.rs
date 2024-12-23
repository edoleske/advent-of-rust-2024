use std::collections::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(23);

fn parse_input(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|l| l.trim().split_once('-').unwrap())
        .map(|(a, b)| (a.to_string(), b.to_string())).collect()
}

fn hash(a: &str, b: &str, c: &str) -> String {
    let mut sorted = vec![a, b, c];
    sorted.sort();
    format!("{}{}{}", sorted[0], sorted[1], sorted[2])
}

fn build_connection_map(edges: &Vec<(String, String)>) -> HashMap<&String, Vec<&String>> {
    let mut connection_map: HashMap<&String, Vec<&String>> = HashMap::new();

    for (pc1, pc2) in edges {
        connection_map.entry(pc1).or_insert(Vec::new()).push(pc2);
        connection_map.entry(pc2).or_insert(Vec::new()).push(pc1);
    }

    for (_, connected) in connection_map.iter_mut() {
        connected.sort();
        connected.dedup();
    }

    connection_map
}

pub fn part_one(input: &str) -> Option<u32> {
    let edges = parse_input(input);
    let connection_map = build_connection_map(&edges);

    let mut result = 0;
    let mut explored: HashSet<String> = HashSet::new();
    for (pc, connected) in &connection_map {
        if connected.len() <= 1 {
            continue;
        }

        for i in 0..connected.len() - 1 {
            let pc1 = &connected[i];
            for pc2 in connected[i + 1..].iter() {
                if !pc.starts_with('t') && !pc1.starts_with('t') && !pc2.starts_with('t') {
                    continue;
                }

                let hash = hash(pc, pc1, pc2);
                if explored.contains(&hash) {
                    continue;
                }
                explored.insert(hash);

                if connection_map[pc1].contains(&pc2)
                    && connection_map[pc1].contains(pc)
                    && connection_map[pc2].contains(&pc1)
                    && connection_map[pc2].contains(pc)
                {
                    result += 1;
                }
            }
        }
    }

    Some(result)
}

fn interconnected(connection_map: &HashMap<&String, Vec<&String>>, nodes: &Vec<&String>) -> bool {
    for node in nodes {
        if nodes.iter().any(|n| n != node && !connection_map[node].contains(n)) {
            return false;
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<String> {
    let edges = parse_input(input);
    let connection_map = build_connection_map(&edges);

    let nodes = connection_map.keys().cloned().collect::<Vec<_>>();
    let mut max: Vec<&String> = Vec::new();
    for node in nodes {
        let mut i = connection_map[&node].len();
        while i > 1 && i + 1 > max.len() {
            for sequence in connection_map[&node].iter().combinations(i) {
                let mut v = vec![node];
                v.extend(sequence);

                if interconnected(&connection_map, &v) {
                    v.sort();
                    max = v;
                }
            }

            i -= 1;
        }
    }

    Some(max.into_iter().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("co,de,ka,ta")));
    }
}
