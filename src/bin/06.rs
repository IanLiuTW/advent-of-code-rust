use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 4))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, 14))
}

fn solve(input: &str, seq_len: usize) -> u32 {
    input
        .lines()
        .map(|line| find_start_index(line, seq_len))
        .sum::<usize>() as u32
}

fn find_start_index(input: &str, seq_len: usize) -> usize {
    let input = input.chars().collect_vec();
    let mut window: HashMap<char, u32> = HashMap::new();

    for (i, c) in input.iter().enumerate() {
        *window.entry(*c).or_default() += 1;

        if let Some(i) = i.checked_sub(seq_len) {
            let c = input[i];

            *window.entry(c).or_default() -= 1;
            if *window.entry(c).or_default() == 0 {
                window.remove(&c);
            }
        }

        if window.keys().len() == seq_len {
            return i + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(39));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(120));
    }
}
