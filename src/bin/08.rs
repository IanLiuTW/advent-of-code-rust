use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = convert_input_to_grid(input);
    let bounds = (grid.len(), grid[0].len());

    let site_locations = get_site_locations(&grid, &bounds);
    let mut all_anti_node_locations: HashSet<(usize, usize)> = HashSet::new();

    for locations in site_locations.values() {
        let anti_node_locations =
            get_anti_node_locations(locations, &bounds, find_and_insert_anti_node_location);
        all_anti_node_locations.extend(&anti_node_locations);
    }

    Some(all_anti_node_locations.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = convert_input_to_grid(input);
    let bounds = (grid.len(), grid[0].len());

    let site_locations = get_site_locations(&grid, &bounds);
    let mut all_anti_node_locations: HashSet<(usize, usize)> = HashSet::new();

    for locations in site_locations.values() {
        let anti_node_locations =
            get_anti_node_locations(locations, &bounds, find_and_insert_anti_node_location2);
        all_anti_node_locations.extend(&anti_node_locations);
    }

    Some(all_anti_node_locations.len() as u32)
}

fn convert_input_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_site_locations(
    grid: &[Vec<char>],
    bounds: &(usize, usize),
) -> HashMap<char, Vec<(usize, usize)>> {
    let mut locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, j) in (0..bounds.0).cartesian_product(0..bounds.1) {
        if grid[i][j] != '.' {
            locations.entry(grid[i][j]).or_default().push((i, j));
        }
    }
    locations
}

fn get_anti_node_locations<F>(
    locations: &[(usize, usize)],
    bounds: &(usize, usize),
    find_and_insert_anti_node_location: F,
) -> HashSet<(usize, usize)>
where
    F: Fn(&(usize, usize), &(usize, usize), &(usize, usize), &mut HashSet<(usize, usize)>),
{
    let mut anti_node_locations: HashSet<(usize, usize)> = HashSet::new();

    for l1 in locations {
        for l2 in locations {
            if l1 == l2 {
                continue;
            }
            find_and_insert_anti_node_location(l1, l2, bounds, &mut anti_node_locations);
        }
    }

    anti_node_locations
}

fn find_and_insert_anti_node_location(
    l1: &(usize, usize),
    l2: &(usize, usize),
    bounds: &(usize, usize),
    anti_node_locations: &mut HashSet<(usize, usize)>,
) {
    let delta = get_delta(l1, l2);
    if let Some(nl) = get_new_location(l2, &delta, bounds) {
        anti_node_locations.insert(nl);
    }
}

fn find_and_insert_anti_node_location2(
    l1: &(usize, usize),
    l2: &(usize, usize),
    bounds: &(usize, usize),
    anti_node_locations: &mut HashSet<(usize, usize)>,
) {
    let delta = get_delta(l1, l2);

    let mut l = *l1;
    while let Some(nl) = get_new_location(&l, &delta, bounds) {
        anti_node_locations.insert(nl);
        l = nl;
    }
}

fn get_delta(l1: &(usize, usize), l2: &(usize, usize)) -> (isize, isize) {
    (l2.0 as isize - l1.0 as isize, l2.1 as isize - l1.1 as isize)
}

fn get_new_location(
    l: &(usize, usize),
    delta: &(isize, isize),
    bounds: &(usize, usize),
) -> Option<(usize, usize)> {
    let nl = (l.0 as isize + delta.0, l.1 as isize + delta.1);

    (
        (0..bounds.0 as isize).contains(&nl.0)
        && (0..bounds.1 as isize).contains(&nl.1)
    )
        .then_some((nl.0 as usize, nl.1 as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
