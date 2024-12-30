advent_of_code::solution!(4);

fn check_diagonal(data: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32) -> bool {
    let first = data[(y as i32 + dy * 1) as usize][(x as i32 + dx * 1) as usize];
    let second = data[(y as i32 + dy * 2) as usize][(x as i32 + dx * 2) as usize];
    let third = data[(y as i32 + dy * 3) as usize][(x as i32 + dx * 3) as usize];
    first == 'M' && second == 'A' && third == 'S'
}

fn count_xmas(data: &Vec<Vec<char>>, x: usize, y: usize, width: usize) -> u32 {
    let mut count = 0;

    if x < width - 3 && data[y][x + 1] == 'M' && data[y][x + 2] == 'A' && data[y][x + 3] == 'S' {
        count += 1;
    }
    if x > 2 && data[y][x - 1] == 'M' && data[y][x - 2] == 'A' && data[y][x - 3] == 'S' {
        count += 1;
    }
    if y < data.len() - 3 && data[y + 1][x] == 'M' && data[y + 2][x] == 'A' && data[y + 3][x] == 'S'
    {
        count += 1;
    }
    if y > 2 && data[y - 1][x] == 'M' && data[y - 2][x] == 'A' && data[y - 3][x] == 'S' {
        count += 1;
    }
    if x < width - 3 && y > 2 {
        if check_diagonal(data, x, y, 1, -1) {
            count += 1;
        }
    }
    if x < width - 3 && y < data.len() - 3 {
        if check_diagonal(data, x, y, 1, 1) {
            count += 1;
        }
    }
    if x > 2 && y > 2 {
        if check_diagonal(data, x, y, -1, -1) {
            count += 1;
        }
    }
    if x > 2 && y < data.len() - 3 {
        if check_diagonal(data, x, y, -1, 1) {
            count += 1;
        }
    }

    count
}

fn count_cross_mas(data: &Vec<Vec<char>>, x: usize, y: usize, width: usize) -> u32 {
    if x < 1 || y < 1 || x > width - 2 || y > data.len() - 2 {
        return 0;
    }

    let tl = data[y - 1][x - 1];
    let tr = data[y - 1][x + 1];
    let bl = data[y + 1][x - 1];
    let br = data[y + 1][x + 1];

    if (tl == 'M' && tr == 'M' && bl == 'S' && br == 'S')
        || (tl == 'M' && tr == 'S' && bl == 'M' && br == 'S')
        || (tl == 'S' && tr == 'S' && bl == 'M' && br == 'M')
        || (tl == 'S' && tr == 'M' && bl == 'S' && br == 'M')
    {
        return 1;
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let width = data[0].len();

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

    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let width = data[0].len();

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
