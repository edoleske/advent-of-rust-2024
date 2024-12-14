advent_of_code::solution!(14);

struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn new(line: &str) -> Self {
        let (ps, vs) = line.trim().split_once(' ').unwrap();
        let (px, py) = ps[2..].split_once(',').unwrap();
        let (vx, vy) = vs[2..].split_once(',').unwrap();

        Robot {
            pos: (px.parse().unwrap(), py.parse().unwrap()),
            vel: (vx.parse().unwrap(), vy.parse().unwrap()),
        }
    }
}

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

struct Bathroom {
    width: i32,
    height: i32,
    robots: Vec<Robot>,
}

impl Bathroom {
    fn new(input: &str) -> Self {
        let robots: Vec<Robot> = input.lines().map(|line| Robot::new(line)).collect();
        let mut bathroom = Bathroom {
            width: 0,
            height: 0,
            robots,
        };

        // Test dimensions are 11x7, while real dimensions are 101x103
        if bathroom.robots.iter().all(|r| r.pos.0 < 11) {
            bathroom.width = 11;
            bathroom.height = 7;
        } else {
            bathroom.width = 101;
            bathroom.height = 103;
        }

        bathroom
    }

    fn advance(&mut self) {
        for robot in &mut self.robots {
            let mut new_pos = (robot.pos.0 + robot.vel.0, robot.pos.1 + robot.vel.1);

            if new_pos.0 < 0 {
                new_pos.0 += self.width;
            } else if new_pos.0 >= self.width {
                new_pos.0 -= self.width;
            }

            if new_pos.1 < 0 {
                new_pos.1 += self.height;
            } else if new_pos.1 >= self.height {
                new_pos.1 -= self.height;
            }

            robot.pos = new_pos;
        }
    }

    fn safety_score(&self) -> u32 {
        let mut tl: u32 = 0;
        let mut tr: u32 = 0;
        let mut bl: u32 = 0;
        let mut br: u32 = 0;

        for robot in &self.robots {
            match self.quadrant(robot) {
                Some(Quadrant::TopLeft) => tl += 1,
                Some(Quadrant::TopRight) => tr += 1,
                Some(Quadrant::BottomLeft) => bl += 1,
                Some(Quadrant::BottomRight) => br += 1,
                None => {}
            }
        }

        tl * tr * bl * br
    }

    fn quadrant(&self, robot: &Robot) -> Option<Quadrant> {
        let x_mid = self.width / 2;
        let y_mid = self.height / 2;

        let left = robot.pos.0 >= 0 && robot.pos.0 < x_mid;
        let right = robot.pos.0 > x_mid && robot.pos.0 < self.width;
        let top = robot.pos.1 >= 0 && robot.pos.1 < y_mid;
        let bottom = robot.pos.1 > y_mid && robot.pos.1 < self.height;

        if left && top {
            return Some(Quadrant::TopLeft);
        } else if right && top {
            return Some(Quadrant::TopRight);
        } else if left && bottom {
            return Some(Quadrant::BottomLeft);
        } else if right && bottom {
            return Some(Quadrant::BottomRight);
        }

        None
    }

    // I found the pattern by looking for long strings of "#####"
    // Then I wrote this to find it without false positives :)
    fn has_pattern(&self) -> bool {
        let mut lines: Vec<String> = (0..self.height)
            .map(|_| (0..self.width).map(|_| '.').collect())
            .collect();
        for robot in &self.robots {
            lines[robot.pos.1 as usize]
                .replace_range(robot.pos.0 as usize..=robot.pos.0 as usize, "#");
        }

        lines
            .iter()
            .any(|line| line.contains("#....#####################....#"))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bathroom = Bathroom::new(input);

    for _ in 0..100 {
        bathroom.advance();
    }

    Some(bathroom.safety_score())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bathroom = Bathroom::new(input);

    for i in 0..10_000 {
        bathroom.advance();

        if bathroom.has_pattern() {
            return Some(i + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
