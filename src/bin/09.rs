advent_of_code::solution!(9);

struct File {
    id: u32,
    index: u32,
    length: u32,
}

impl File {
    fn new(id: u32, index: u32, length: u32) -> File {
        File { id, index, length }
    }

    fn decrement(&mut self) {
        if self.length <= 0 {
            panic!("Attempting to decrement file with of 0 length")
        }

        self.length -= 1;
    }
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect()
}

fn parse_files(nums: &Vec<u32>) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    let mut i: u32 = 0;

    for (id, &x) in nums.iter().enumerate() {
        if id % 2 == 0 {
            files.push(File::new(id as u32 / 2, i, x));
        }
        i += x;
    }

    files
}

fn parse_free_blocks(input: &Vec<u32>) -> Vec<(u32, u32)> {
    let mut free_blocks: Vec<(u32, u32)> = Vec::new();

    let mut i: u32 = 0;
    for (ni, &n) in input.iter().enumerate() {
        if ni % 2 == 0 {
            i += n;
            continue;
        }

        free_blocks.push((i, n));

        i += n;
    }

    free_blocks
}

fn get_files_checksum(files: &Vec<File>) -> u64 {
    let mut checksum = 0;

    for f in files {
        for x in f.index..f.index + f.length {
            checksum += (f.id * x) as u64;
        }
    }

    checksum
}

pub fn part_one(input: &str) -> Option<u64> {
    let input_nums = parse_input(input);
    let mut files = parse_files(&input_nums);

    let mut checksum: u64 = 0;

    // Rearrange and add moved blocks to checksum
    let mut i: u32 = 0;
    for (ni, &n) in input_nums.iter().enumerate() {
        if ni % 2 == 0 {
            i += n;
            continue;
        }

        for x in i..i + n {
            let last_file = files.last_mut().unwrap();
            if last_file.index < x {
                break;
            }

            checksum += (last_file.id * x) as u64;
            last_file.decrement();

            if last_file.length == 0 {
                files.remove(files.len() - 1);
            }
        }

        i += n;
    }

    checksum += get_files_checksum(&files);

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_nums = parse_input(input);
    let mut files = parse_files(&input_nums);
    let mut free_blocks: Vec<(u32, u32)> = parse_free_blocks(&input_nums);

    for file in files.iter_mut().rev() {
        if let Some((i, l)) = free_blocks
            .iter_mut()
            .filter(|fb| fb.0 < file.index && fb.1 >= file.length)
            .next()
        {
            file.index = *i;

            let new_length = *l - file.length;
            if new_length > 0 {
                *i += file.length;
            }
            *l = new_length;
        }
    }

    Some(get_files_checksum(&files))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
