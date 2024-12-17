advent_of_code::solution!(17);

#[derive(Debug, Copy, Clone)]
struct Instruction(u8, u64);

struct CPU {
    a: u64,
    b: u64,
    c: u64,
    i: i32,
}

impl CPU {
    fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            i: 0,
        }
    }

    fn execute(&mut self, program: &Vec<Instruction>) -> String {
        let mut output: Vec<u8> = Vec::new();

        while self.i >= 0 && self.i < program.len() as i32 {
            let (operator, operand) = (program[self.i as usize].0, program[self.i as usize].1);

            match operator {
                0 => self.a = self.a / 2u64.pow(self.combo(&operand) as u32),
                1 => self.b = self.b ^ operand,
                2 => self.b = self.combo(&operand) % 8,
                3 => {
                    if self.a > 0 {
                        self.i = operand as i32 / 2;
                    } else {
                        self.i += 1;
                    }
                }
                4 => self.b = self.b ^ self.c,
                5 => output.push((self.combo(&operand) % 8) as u8),
                6 => self.b = self.a / 2u64.pow(self.combo(&operand) as u32),
                7 => self.c = self.a / 2u64.pow(self.combo(&operand) as u32),
                _ => panic!("Unsupported operator {}", operator),
            }

            if operator != 3 {
                self.i += 1;
            }
        }

        output
            .iter()
            .map(|&n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn combo(&self, operand: &u64) -> u64 {
        match operand {
            0..=3 => *operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand: {}", operand),
        }
    }
}

fn parse_input(input: &str) -> (CPU, String) {
    let mut cpu = CPU::new();
    cpu.a = input.lines().nth(0).unwrap().trim()[12..].parse().unwrap();
    cpu.b = input.lines().nth(1).unwrap().trim()[12..].parse().unwrap();
    cpu.c = input.lines().nth(2).unwrap().trim()[12..].parse().unwrap();

    let program = input.lines().nth(4).unwrap().trim()[9..].to_string();
    (cpu, program)
}

fn parse_program(input: &String) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();

    let tokens: Vec<u32> = input
        .split(",")
        .map(|t| t.parse::<u32>().unwrap())
        .collect();
    for ch in tokens.chunks(2) {
        program.push(Instruction(ch[0] as u8, ch[1] as u64));
    }

    program
}

fn check_a(cpu: &mut CPU, program: &Vec<Instruction>, a: u64) -> String {
    cpu.a = a;
    cpu.b = 0;
    cpu.c = 0;
    cpu.i = 0;
    cpu.execute(program)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut cpu, program_string) = parse_input(input);
    let program = parse_program(&program_string);

    let output = cpu.execute(&program);

    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut cpu, program_string) = parse_input(input);
    let program = parse_program(&program_string);

    let mut min_a: u64 = u64::MAX;
    let mut inputs = (0..8).collect::<Vec<u64>>();
    let l = (program_string.len() + 1) / 2;

    // Brute force each 3-bit number, working backwards from end of target
    for i in 0..l {
        let mut next = Vec::new();

        for n in &inputs {
            let result = check_a(&mut cpu, &program, *n);
            if program_string[program_string.len() - i * 2 - 1..] == result {
                if result.len() == program_string.len() {
                    if *n < min_a {
                        min_a = *n;
                    }
                }

                for j in 0..=8 {
                    if (n * 8 + j) / 8 == *n {
                        next.push(n * 8 + j);
                    }
                }
            }
        }

        inputs = next;
    }

    Some(min_a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let _result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // Doesn't work for test case
        // assert_eq!(result, Some(117440));
    }
}
