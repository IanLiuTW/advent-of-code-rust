use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = 0;

    for (left, right) in parse_input(input) {
        let left: HashSet<char> = left.chars().collect();
        let right: HashSet<char> = right.chars().collect();

        ans += get_priority(*left.intersection(&right).next().unwrap());
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans = 0;

    for group in parse_input2(input) {
        let mut hs: HashSet<char> = group[0].chars().collect();
        hs.retain(|&c| group[1].contains(c));
        hs.retain(|&c| group[2].contains(c));

        ans += get_priority(*hs.iter().next().unwrap());
    }

    Some(ans)
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let line = line.split_at(line.len() / 2);
            (line.0, line.1)
        })
        .collect::<Vec<(&str, &str)>>()
}

fn parse_input2(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect()
}

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(70));
    }
}
