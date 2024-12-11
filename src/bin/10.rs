use std::{collections::HashSet, usize};

use itertools::Itertools;
advent_of_code::solution!(10);

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn part_one(input: &str) -> Option<u32> {
    let grid = convert_input_to_grid(input);
    let bounds = (grid.len(), grid[0].len());

    let trail_heads = find_trail_heads(&grid, &bounds);

    let mut ans = 0;
    for l in trail_heads {
        ans += dfs(l, &grid, &bounds, &mut HashSet::new());
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = convert_input_to_grid(input);
    let bounds = (grid.len(), grid[0].len());

    let trail_heads = find_trail_heads(&grid, &bounds);

    let mut ans = 0;
    for l in trail_heads {
        ans += dfs2(l, &grid, &bounds);
    }

    Some(ans)
}

fn convert_input_to_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_trail_heads(grid: &Vec<Vec<u32>>, bounds: &(usize, usize)) -> Vec<(usize, usize)> {
    (0..bounds.0)
        .cartesian_product(0..bounds.1)
        .filter(|l| grid[l.0][l.1] == 0)
        .collect()
}

fn dfs(
    l: (usize, usize),
    grid: &[Vec<u32>],
    bounds: &(usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> u32 {
    if visited.contains(&l) {
        return 0;
    }
    visited.insert(l);

    if grid[l.0][l.1] == 9 {
        return 1;
    }

    let mut ans = 0;
    for delta in DIRECTIONS {
        if let Some(nl) = move_to(&l, &delta, bounds) {
            if grid[nl.0][nl.1] == grid[l.0][l.1] + 1 {
                ans += dfs(nl, grid, bounds, visited);
            }
        }
    }

    ans
}

fn dfs2(
    l: (usize, usize),
    grid: &[Vec<u32>],
    bounds: &(usize, usize),
) -> u32 {
    if grid[l.0][l.1] == 9 {
        return 1;
    }

    let mut ans = 0;
    for delta in DIRECTIONS {
        if let Some(nl) = move_to(&l, &delta, bounds) {
            if grid[nl.0][nl.1] == grid[l.0][l.1] + 1 {
                ans += dfs2(nl, grid, bounds);
            }
        }
    }

    ans
}

fn move_to(
    l: &(usize, usize),
    delta: &(isize, isize),
    bounds: &(usize, usize),
) -> Option<(usize, usize)> {
    let nl = (l.0 as isize + delta.0, l.1 as isize + delta.1);

    ((0..bounds.0 as isize).contains(&nl.0) && (0..bounds.1 as isize).contains(&nl.1))
        .then_some((nl.0 as usize, nl.1 as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {


        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
