advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Invalid direction"),
        }
    }

    fn increment(&self, position: &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::Right => (position.0 + 1, position.1),
            Direction::Left => (position.0 - 1, position.1),
            Direction::Up => (position.0, position.1 - 1),
            Direction::Down => (position.0, position.1 + 1),
        }
    }
}

struct Warehouse {
    data: Vec<Vec<char>>,
    robot: (usize, usize),
    sequence: Vec<Direction>,
    temp: String,
}

impl Warehouse {
    fn new(input: &str) -> Self {
        let mut data: Vec<Vec<char>> = Vec::new();
        let mut robot: (usize, usize) = (0, 0);
        let mut sequence: String = String::new();

        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }

            if line.contains('#') {
                if line.contains('@') {
                    robot = (
                        line.trim().chars().position(|c| c == '@').unwrap(),
                        data.len(),
                    );
                }

                data.push(line.trim().chars().collect());
            } else {
                sequence += line.trim();
            }
        }

        Warehouse {
            data,
            robot,
            sequence: sequence.chars().map(|c| Direction::from_char(c)).collect(),
            temp: String::new(),
        }
    }

    fn double(&mut self) {
        let mut new_lines = Vec::with_capacity(self.data.len());

        for line in &self.data {
            let mut new_line = String::with_capacity(line.len() * 2);
            for c in line {
                match c {
                    '#' => new_line += "##",
                    'O' => new_line += "[]",
                    '.' => new_line += "..",
                    '@' => {
                        self.robot = (new_line.len(), new_lines.len());
                        new_line += "@.";
                    }
                    _ => {}
                }
            }

            new_lines.push(new_line.chars().collect());
        }

        self.data = new_lines;
    }

    fn get(&self, index: (usize, usize)) -> &char {
        &self.data[index.1][index.0]
    }

    fn swap(&mut self, first: (usize, usize), second: (usize, usize)) {
        let temp = *self.get(first);
        self.data[first.1][first.0] = self.data[second.1][second.0];
        self.data[second.1][second.0] = temp;
        self.temp = format!("{}", self);
    }

    fn robot_move(&mut self, to: (usize, usize)) {
        self.swap(to, self.robot);
        self.robot = to;
    }

    // Attempt to push box and return whether successful or not
    fn push(&mut self, index: (usize, usize), direction: &Direction) -> bool {
        let mut swap_position = index;
        while self.get(swap_position) == &'O' {
            swap_position = direction.increment(&swap_position);
        }

        if self.get(swap_position) != &'#' {
            self.swap(index, swap_position);
            return true;
        }

        false
    }

    fn sum_boxes(&self) -> u32 {
        let mut sum = 0;

        for y in 0..self.data.len() {
            for x in 0..self.data.iter().next().unwrap().len() {
                if self.data[y][x] == 'O' || self.data[y][x] == '[' {
                    sum += 100 * y as u32 + x as u32;
                }
            }
        }

        sum
    }
}

impl std::fmt::Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s: String = self
            .data
            .iter()
            .map(|line| line.iter().collect::<String>() + "\n")
            .collect();
        write!(f, "{}", s)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut warehouse = Warehouse::new(input);
    let sequence = warehouse.sequence.clone();

    for next in sequence.iter() {
        let next_position = next.increment(&warehouse.robot);

        match warehouse.get(next_position) {
            '#' => continue,
            'O' => {
                if warehouse.push(next_position, next) {
                    warehouse.robot_move(next_position);
                }
            }
            _ => warehouse.robot_move(next_position),
        }
    }

    Some(warehouse.sum_boxes())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
