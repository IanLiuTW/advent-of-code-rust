use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(()),
        }
    }
}

struct Grid<'a> {
    lines: Vec<&'a str>,
    width: usize,
    height: usize,
}

impl<'a> Grid<'a> {
    fn new(input: &'a str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let height = lines.len();

        Self {
            lines,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.lines
            .get(y)
            .and_then(|line| line.as_bytes().get(x))
            .map(|&b| b as char)
            .unwrap_or(' ')
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let ops_row = lines.pop()?;
    let operators: Vec<Op> = ops_row
        .iter()
        .filter_map(|s| Op::from_str(s).ok())
        .collect();

    let cols = lines[0].len();
    let ans: u64 = (0..cols)
        .map(|i| {
            let nums = lines.iter().map(|row| row[i].parse::<u64>().unwrap());
            let op = operators[i];

            nums.reduce(|a, b| op.apply(a, b)).unwrap()
        })
        .sum();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    let number_rows = grid.height - 1;

    let mut total_sum = 0;
    let mut current_nums: Vec<u64> = Vec::new();

    for x in (0..grid.width).rev() {
        let is_separator = (0..number_rows).all(|y| grid.get(x, y).is_whitespace());

        if !is_separator {
            let mut num_str = String::with_capacity(number_rows);

            for y in 0..number_rows {
                let c = grid.get(x, y);
                if !c.is_whitespace() {
                    num_str.push(c);
                }
            }

            if let Ok(num) = num_str.parse::<u64>() {
                current_nums.push(num);
            }

            let char_at_bottom = grid.get(x, grid.height - 1);
            if let Ok(op) = Op::from_str(&char_at_bottom.to_string()) {
                total_sum += current_nums
                    .iter()
                    .cloned()
                    .reduce(|a, b| op.apply(a, b))
                    .unwrap_or(0);

                current_nums.clear();
            }
        }
    }

    Some(total_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
