advent_of_code::solution!(13);

struct Machine {
    target: (i32, i32),
    a: (i32, i32),
    b: (i32, i32),
}

impl Machine {
    fn new() -> Machine {
        Machine { target: (0, 0), a: (0, 0), b: (0, 0) }
    }
    
    fn get_token_count(&self) -> u32 {
        let mut tokens: u32 = 0;
        
        for i in 1..101 {
            let (xi, yi) = (self.target.0 - self.a.0 * i, self.target.1 - self.a.1 * i);
            
            if xi <= 0 || yi <= 0 {
                break;
            }
            
            if xi % self.b.0 == 0 && yi % self.b.1 == 0 {
                let xb = xi / self.b.0;
                if xb == yi / self.b.1 {
                    tokens = (3 * i + xb) as u32;
                    break;
                }
            }
        }
        
        tokens
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
            machine.target.0 = xs.trim().split_once('=').unwrap().1.parse::<i32>().unwrap();
            machine.target.1 = ys.trim().split_once('=').unwrap().1.parse::<i32>().unwrap();
        }
        
        if line.starts_with("Button A: ") {
            let (xs, ys) = line[10..].trim().split_once(',').unwrap();
            machine.a.0 = xs.trim().split_once('+').unwrap().1.parse::<i32>().unwrap();
            machine.a.1 = ys.trim().split_once('+').unwrap().1.parse::<i32>().unwrap();
        }
        
        if line.starts_with("Button B: ") {
            let (xs, ys) = line[10..].trim().split_once(',').unwrap();
            machine.b.0 = xs.trim().split_once('+').unwrap().1.parse::<i32>().unwrap();
            machine.b.1 = ys.trim().split_once('+').unwrap().1.parse::<i32>().unwrap();
        }
    }
    
    machines.push(machine);
    machines
} 

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let machines = parse_input(input);
    
    for machine in machines {
        let count = machine.get_token_count();
        result += count;
    }
    
    Some(result)
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
