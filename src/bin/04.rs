use advent_of_code::grid::Grid;
use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(4);

const VALID_NEIGHBOR_LIMIT: usize = 4;

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let occupied_pos = get_occupied_pos_from_grid(&grid);
    let total_cnt = get_valid_pos_form_iter(&grid, occupied_pos).count();

    Some(total_cnt as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_input(input);
    let mut occupied_pos = get_occupied_pos_from_grid(&grid).collect::<HashSet<(usize, usize)>>();
    let mut total_cnt = 0;

    loop {
        let valid_pos = get_valid_pos_form_iter(&grid, occupied_pos.iter().copied()).collect_vec();

        if valid_pos.is_empty() {
            break;
        }

        for &(i, j) in valid_pos.iter() {
            grid.set_pos_val(i, j, false).ok();
            occupied_pos.remove(&(i, j));
        }

        total_cnt += valid_pos.len();
    }

    Some(total_cnt as u64)
}

fn parse_input(input: &str) -> Grid<bool> {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect_vec())
        .collect_vec();

    Grid::new(grid)
}

fn get_occupied_pos_from_grid(grid: &Grid<bool>) -> impl Iterator<Item = (usize, usize)> {
    grid.iter_all_pos_in_grid()
        .filter(|&(i, j)| grid.get_pos_val(i, j).unwrap_or(false))
}

fn get_valid_pos_form_iter(
    grid: &Grid<bool>,
    iter: impl Iterator<Item = (usize, usize)>,
) -> impl Iterator<Item = (usize, usize)> {
    iter.filter(|&(i, j)| {
        grid.iter_eight_neighbors_of_pos(i, j)
            .filter(|&item| item)
            .count()
            < VALID_NEIGHBOR_LIMIT
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
