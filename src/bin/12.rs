use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(12);

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = convert_input_to_grid(input);
    let bounds = (grid.len(), grid[0].len());

    Some(get_fence_cost(
        &mut grid,
        &bounds,
        compute_cost_and_update_grid,
    ))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = convert_input_to_grid(input);
    let bounds = (grid.len(), grid[0].len());

    Some(get_fence_cost(
        &mut grid,
        &bounds,
        compute_cost_and_update_grid2,
    ))
}

fn convert_input_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_fence_cost<F>(
    grid: &mut Vec<Vec<char>>,
    bounds: &(usize, usize),
    compute_cost_and_update_grid: F,
) -> u32
where
    F: Fn(&mut Vec<Vec<char>>, &(usize, usize), HashSet<(usize, usize)>) -> u32,
{
    let mut cost = 0;

    for pos in (0..bounds.0).cartesian_product(0..bounds.1) {
        if grid[pos.0][pos.1] == ' ' {
            continue;
        }

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        dfs(&pos, grid, bounds, &mut visited);

        cost += compute_cost_and_update_grid(grid, bounds, visited);
    }

    cost
}

fn dfs(
    pos: &(usize, usize),
    grid: &mut Vec<Vec<char>>,
    bounds: &(usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) {
    if visited.contains(&pos) {
        return;
    }
    visited.insert(*pos);

    for delta in DIRECTIONS {
        if let Some(next_pos) = is_valid_pos(pos, bounds, &delta) {
            if grid[next_pos.0][next_pos.1] != grid[pos.0][pos.1] {
                continue;
            }
            dfs(&next_pos, grid, bounds, visited);
        }
    }
}

fn compute_cost_and_update_grid(
    grid: &mut Vec<Vec<char>>,
    bounds: &(usize, usize),
    visited: HashSet<(usize, usize)>,
) -> u32 {
    let mut perimeter = 0;

    for pos in &visited {
        for delta in DIRECTIONS {
            if let Some(next_pos) = is_valid_pos(pos, bounds, &delta) {
                if grid[next_pos.0][next_pos.1] != grid[pos.0][pos.1] {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    for pos in &visited {
        grid[pos.0][pos.1] = ' ';
    }

    visited.len() as u32 * perimeter
}

fn compute_cost_and_update_grid2(
    grid: &mut Vec<Vec<char>>,
    bounds: &(usize, usize),
    visited: HashSet<(usize, usize)>,
) -> u32 {
    let mut borders: HashMap<(isize, isize), Vec<(usize, usize)>> = HashMap::new();
    for pos in &visited {
        for delta in DIRECTIONS {
            if let Some(next_pos) = is_valid_pos(pos, bounds, &delta) {
                if grid[next_pos.0][next_pos.1] != grid[pos.0][pos.1] {
                    borders.entry(delta).or_default().push(*pos);
                }
            } else {
                borders.entry(delta).or_default().push(*pos);
            }
        }
    }

    let mut sides = 0;
    for delta in DIRECTIONS {
        if let Some(v) = borders.get(&delta) {
            let mut borders: HashMap<usize, Vec<usize>> = HashMap::new();
            for pos in v {
                if delta.1 == 0 {
                    borders.entry(pos.0).or_default().push(pos.1);
                } else {
                    borders.entry(pos.1).or_default().push(pos.0);
                }
            }
            for border in borders.values_mut() {
                border.sort();

                let mut segments = 0;
                for i in 0..border.len() {
                    if i == 0 || border[i] != border[i - 1] + 1 {
                        segments += 1
                    }
                }
                sides += segments;
            }
        }
    }

    for pos in &visited {
        grid[pos.0][pos.1] = ' ';
    }

    sides * visited.len() as u32
}

fn is_valid_pos(
    pos: &(usize, usize),
    bounds: &(usize, usize),
    delta: &(isize, isize),
) -> Option<(usize, usize)> {
    let ni = pos.0 as isize + delta.0;
    let nj = pos.1 as isize + delta.1;
    ((0..bounds.0 as isize).contains(&ni) && (0..bounds.1 as isize).contains(&nj))
        .then_some((ni as usize, nj as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
