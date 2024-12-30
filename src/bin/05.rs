use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, operations) = parse_input(input);

    for [amount, from, to] in operations {
        for _ in 0..amount {
            if let Some(c) = stacks[from].pop_back() {
                stacks[to].push_back(c);
            }
        }
    }

    Some(get_top_chars(&stacks))
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, operations) = parse_input(input);

    for [amount, from, to] in operations {
        let mut tmp = vec![];
        for _ in 0..amount {
            if let Some(c) = stacks[from].pop_back() {
                tmp.push(c);
            }
        }
        while let Some(c) = tmp.pop() {
            stacks[to].push_back(c);
        }
    }

    Some(get_top_chars(&stacks))
}

fn parse_input(input: &str) -> (Vec<VecDeque<char>>, Vec<[usize; 3]>) {
    let mut input = input.split("\n\n");
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];
    let mut operations: Vec<[usize; 3]> = vec![];

    let part1 = input.next().unwrap();
    part1.lines().for_each(|line| {
        for (i, chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let mut chunk = chunk.into_iter();
            if chunk.next().is_some_and(|c| c == '[') {
                if let Some(c) = chunk.next() {
                    stacks[i].push_front(c);
                }
            }
        }
    });

    let part2 = input.next().unwrap();
    let re_line = Regex::new(r#"move (\d*) from (\d*) to (\d*)"#).unwrap();
    part2.lines().for_each(|line| {
        let caps = re_line.captures(line).unwrap();
        operations.push([
            caps[1].parse().unwrap(),
            caps[2].parse::<usize>().unwrap() - 1,
            caps[3].parse::<usize>().unwrap() - 1,
        ]);
    });

    (stacks, operations)
}

fn get_top_chars(stacks: &Vec<VecDeque<char>>) -> String {
    let ans = stacks
        .iter()
        .filter_map(|stack| stack.back())
        .collect::<String>();
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("MCD".to_string()));
    }
}
