advent_of_code::solution!(13);

struct Machine {
    target: (i64, i64),
    a: (i64, i64),
    b: (i64, i64),
}

impl Machine {
    fn new() -> Machine {
        Machine {
            target: (0, 0),
            a: (0, 0),
            b: (0, 0),
        }
    }

    // Get the token count by solving the system of equations
    // a0x + b0y = c0
    // a1x + b1y = c1
    // y = (a1c0 - a0c1) - (a1b0 - a0b1)
    // x = (c0 - b0y) / a0
    fn get_token_count(&self) -> u64 {
        let b_nominator = self.a.1 * self.target.0 - self.a.0 * self.target.1;
        let b_denominator = self.a.1 * self.b.0 - self.a.0 * self.b.1;
        if b_nominator % b_denominator != 0 {
            return 0;
        }
        let b = b_nominator / b_denominator;

        let a_nominator = self.target.0 - self.b.0 * b;
        if a_nominator % self.a.0 != 0 {
            return 0;
        }
        let a0 = a_nominator / self.a.0;

        // This may be unnecessary, but it prevents overflows and false positives
        let a1 = (self.target.1 - self.b.1 * b) / self.a.1;
        if b <= 0 || a0 <= 0 || a0 != a1 {
            return 0;
        }

        a0 as u64 * 3 + b as u64
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();
    let mut machine = Machine::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            machines.push(machine);
            machine = Machine::new();
            continue;
        }

        if line.starts_with("Prize: ") {
            let (xs, ys) = line[7..].trim().split_once(',').unwrap();
            machine.target.0 = xs.trim().split_once('=').unwrap().1.parse::<i64>().unwrap();
            machine.target.1 = ys.trim().split_once('=').unwrap().1.parse::<i64>().unwrap();
        }

        if line.starts_with("Button A: ") {
            let (xs, ys) = line[10..].trim().split_once(',').unwrap();
            machine.a.0 = xs.trim().split_once('+').unwrap().1.parse::<i64>().unwrap();
            machine.a.1 = ys.trim().split_once('+').unwrap().1.parse::<i64>().unwrap();
        }

        if line.starts_with("Button B: ") {
            let (xs, ys) = line[10..].trim().split_once(',').unwrap();
            machine.b.0 = xs.trim().split_once('+').unwrap().1.parse::<i64>().unwrap();
            machine.b.1 = ys.trim().split_once('+').unwrap().1.parse::<i64>().unwrap();
        }
    }

    machines.push(machine);
    machines
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    let machines = parse_input(input);

    for machine in machines {
        let count = machine.get_token_count();
        result += count;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    let mut machines = parse_input(input);

    for machine in machines.iter_mut() {
        let (tx, ty) = machine.target;
        machine.target.0 = tx + 10_000_000_000_000i64;
        machine.target.1 = ty + 10_000_000_000_000i64;

        let count = machine.get_token_count();
        result += count;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
