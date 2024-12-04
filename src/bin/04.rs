advent_of_code::solution!(4);

fn count_xmas(data: &Vec<Vec<char>>, x: usize, y: usize, width: usize) -> u32 {
    let mut substrings: Vec<String> = Vec::new();

    if x < width - 3 {
        substrings.push(data[y][x..=x+3].iter().collect::<String>());
    }
    if x > 2 {
        substrings.push(data[y][x-3..=x].iter().rev().collect::<String>());
    }
    if y < data.len() - 3 {
        substrings.push(data[y..=y+3].iter().map(|line| line[x]).collect::<String>());
    }
    if y > 2 {
        substrings.push(data[y - 3..=y].iter().rev().map(|line| line[x]).collect::<String>());
    }
    if x < width - 3 && y > 2 {
        let mut substring: String = String::default();
        for i in 0..4 {
            substring.push(data[y-i][x+i]);
        }
        substrings.push(substring);
    }
    if x < width - 3 && y < data.len() - 3 {
        let mut substring: String = String::default();
        for i in 0..4 {
            substring.push(data[y+i][x+i]);
        }
        substrings.push(substring);
    }
    if x > 2 && y > 2 {
        let mut substring: String = String::default();
        for i in 0..4 {
            substring.push(data[y-i][x-i]);
        }
        substrings.push(substring);
    }
    if x > 2 && y < data.len() - 3 {
        let mut substring: String = String::default();
        for i in 0..4 {
            substring.push(data[y+i][x-i]);
        }
        substrings.push(substring);
    }

    substrings.iter().filter(|&s| s == "XMAS").count() as u32
}

fn count_cross_mas(data: &Vec<Vec<char>>, x: usize, y: usize, width: usize) -> u32 {
    if x < 1 || y < 1 || x > width - 2 || y > data.len() - 2 {
        return 0;
    }
    
    let tl = data[y-1][x-1];
    let tr = data[y-1][x+1];
    let bl = data[y+1][x-1];
    let br = data[y+1][x+1];
    
    if (tl == 'M' && tr == 'M' && bl == 'S' && br == 'S') ||
        (tl == 'M' && tr == 'S' && bl == 'M' && br == 'S') ||
        (tl == 'S' && tr == 'S' && bl == 'M' && br == 'M') ||
        (tl == 'S' && tr == 'M' && bl == 'S' && br == 'M') {
        return 1;
    }
    
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    let data: Vec<Vec<char>> = input.lines().map(|line| line.trim().chars().collect()).collect();

    // Assert input is even grid
    let width = data[0].len();
    if !data.iter().all(|line| line.len() == width) {
        panic!("Input is uneven");
    }

    for y in 0..data.len() {
        for x in 0..width {
            if data[y][x] == 'X' {
                total += count_xmas(&data, x, y, width);
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    let data: Vec<Vec<char>> = input.lines().map(|line| line.trim().chars().collect()).collect();

    // Assert input is even grid
    let width = data[0].len();
    if !data.iter().all(|line| line.len() == width) {
        panic!("Input is uneven");
    }

    for y in 0..data.len() {
        for x in 0..width {
            if data[y][x] == 'A' {
                total += count_cross_mas(&data, x, y, width);
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
