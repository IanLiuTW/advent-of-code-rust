use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = convert_input_to_grid(input);
    let bounds = (grid.len(), grid[0].len());
    let starting_pos = get_starting_pos(&grid, &bounds);

    let visited = simulate_path(&mut grid, &bounds, starting_pos);

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = convert_input_to_grid(input);
    let bounds = (grid.len(), grid[0].len());
    let starting_pos = get_starting_pos(&grid, &bounds);

    let mut loop_positions = 0;
    for (i, j) in (0..bounds.0).cartesian_product(0..bounds.1) {
        if grid[i][j] == '.' && (i, j) != starting_pos {
            grid[i][j] = '#';
            if will_stuck_in_loop(&grid, &bounds, starting_pos) {
                loop_positions += 1;
            }
            grid[i][j] = '.';
        }
    }

    Some(loop_positions)
}

fn simulate_path(
    grid: &mut [Vec<char>],
    bounds: &(usize, usize),
    starting_pos: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut pos = starting_pos;
    let mut direction: usize = 0;
    let mut visited = HashSet::new();

    loop {
        visited.insert(pos);

        let delta = DIRECTIONS[direction];
        if let Some(next_pos) = get_next_move(&pos, &delta, bounds) {
            if grid[next_pos.0][next_pos.1] == '#' {
                direction = (direction + 1) % 4;
            } else {
                pos = next_pos;
            }
        } else {
            break;
        }
    }

    visited
}

fn will_stuck_in_loop(
    grid: &[Vec<char>],
    bounds: &(usize, usize),
    starting_pos: (usize, usize),
) -> bool {
    let mut pos = starting_pos;
    let mut direction = 0;
    let mut visited = HashSet::new();

    loop {
        let state = (pos, direction);
        if !visited.insert(state) {
            return true;
        }

        let delta = DIRECTIONS[direction];
        if let Some(next_pos) = get_next_move(&pos, &delta, bounds) {
            if grid[next_pos.0][next_pos.1] == '#' {
                direction = (direction + 1) % 4;
            } else {
                pos = next_pos;
            }
        } else {
            return false;
        }
    }
}

fn convert_input_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_starting_pos(grid: &[Vec<char>], bounds: &(usize, usize)) -> (usize, usize) {
    for pos in (0..bounds.0).cartesian_product(0..bounds.1) {
        if grid[pos.0][pos.1] == '^' {
            return pos;
        }
    }
    unreachable!()
}

fn get_next_move(
    pos: &(usize, usize),
    delta: &(isize, isize),
    bounds: &(usize, usize),
) -> Option<(usize, usize)> {
    let ni = pos.0 as isize + delta.0;
    let nj = pos.1 as isize + delta.1;
    if (0..bounds.0 as isize).contains(&ni) && (0..bounds.1 as isize).contains(&nj) {
        Some((ni as usize, nj as usize))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
