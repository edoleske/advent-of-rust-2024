use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

type NodeHashMap = HashMap<char, Vec<(i32, i32)>>;

struct Board {
    width: usize,
    height: usize,
    nodes: NodeHashMap,
}

impl Board {
    fn valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }
    
    fn get_node_positions(&self) -> Vec<(i32, i32)> {
        self.nodes.iter().flat_map(|(_, p)| p.clone()).collect()
    }
}

fn parse_input(input: &str) -> Board {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut nodes: NodeHashMap = HashMap::new();

    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c != '.' {
                nodes
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((i as i32, j as i32));
            }
        }
    }

    Board {
        width,
        height,
        nodes,
    }
}

fn get_antinodes(board: &Board, pos1: (i32, i32), pos2: (i32, i32)) -> Vec<(i32, i32)> {
    let mut result = Vec::new();

    let difference = (pos1.0 - pos2.0, pos1.1 - pos2.1);

    let antinode1 = (pos1.0 + difference.0, pos1.1 + difference.1);
    if board.valid(antinode1.0, antinode1.1) {
        result.push(antinode1);
    }

    let antinode2 = (pos2.0 - difference.0, pos2.1 - difference.1);
    if board.valid(antinode2.0, antinode2.1) {
        result.push(antinode2);
    }

    result
}

fn get_resonant_nodes(board: &Board, pos1: (i32, i32), pos2: (i32, i32)) -> Vec<(i32, i32)> {
    let mut result = Vec::new();

    let difference = (pos1.0 - pos2.0, pos1.1 - pos2.1);

    for i in 1..board.height {
        let (dx, dy) = (difference.0 * i as i32, difference.1 * i as i32);
        let antinodes = vec![(pos1.0 + dx, pos1.1 + dy), (pos2.0 - dx, pos2.1 - dy)];

        if antinodes.iter().all(|(x, y)| !board.valid(*x, *y)) {
            break;
        }

        for (x, y) in antinodes {
            if board.valid(x, y) && pos1 != (x, y) && pos2 != (x, y) {
                result.push((x, y));
            }
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = parse_input(input);
    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    for (_, nodes) in &board.nodes {
        for i in 0..nodes.len() - 1 {
            for other in nodes[i + 1..].iter() {
                let antinodes = get_antinodes(&board, nodes[i], *other);
                positions.extend(antinodes);
            }
        }
    }

    Some(positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = parse_input(input);
    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    for (_, nodes) in &board.nodes {
        for i in 0..nodes.len() - 1 {
            for other in nodes[i + 1..].iter() {
                let antinodes = get_resonant_nodes(&board, nodes[i], *other);
                positions.extend(antinodes);
            }
        }
    }
    
    positions.extend(&board.get_node_positions());

    Some(positions.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
