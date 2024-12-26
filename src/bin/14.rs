use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
};

use itertools::Itertools;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = get_map(input);
    map = tilt_north(map);
    Some(find_total_load(&map) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = get_map(input);
    let mut cache = HashSet::new();
    let target_cycle = 1_000_000_000;
    let mut cycle_start_idx = 0;
    let mut cycle_detected = false;

    let mut i = 0;
    while i < target_cycle {
        map = cycle(map);
        let h = get_hash(&map);
        if !cache.contains(&h) {
            cache.insert(h);
        } else if !cycle_detected {
            cycle_detected = true;
            cycle_start_idx = i;
            cache.clear();
            cache.insert(h);
        } else {
            let cycle_len = cache.len();
            i = cycle_start_idx + ((target_cycle - cycle_start_idx) / cycle_len) * cycle_len;
            cache.clear();
        }
        i += 1;
    }
    Some(find_total_load(&map) as u32)
}

fn get_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|row| row.chars().collect_vec())
        .collect_vec()
}

fn get_hash(map: &Vec<Vec<char>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    map.hash(&mut hasher);
    hasher.finish()
}

fn slide_north(map: &mut Vec<Vec<char>>, mut i: usize, j: usize) {
    if i == 0 || ['.', '#'].contains(&map[i][j]) {
        return;
    }
    map[i][j] = '.';
    while i > 0 && map[i - 1][j] == '.' {
        i -= 1;
    }
    map[i][j] = 'O';
}

fn find_total_load(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|c| **c == 'O').count() * (map.len() - i))
        .sum()
}

fn tilt_north(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            slide_north(&mut map, i, j);
        }
    }
    map
}

fn rotate_clockwise(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = vec![];
    for j in 0..map[0].len() {
        new_map.push((0..map.len()).rev().map(|i| map[i][j]).collect_vec());
    }
    new_map
}

fn cycle(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for _ in 0..4 {
        map = tilt_north(map);
        map = rotate_clockwise(map);
    }
    map
}

fn visualize(map: &Vec<Vec<char>>) {
    for _ in map
        .iter()
        .inspect(|&row| println!("{}", row.iter().collect::<String>()))
    {}
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn visualize_map() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let mut map = get_map(input);
        visualize(&map);
        map = cycle(map);
        visualize(&map);
        map = cycle(map);
        visualize(&map);
        map = cycle(map);
        visualize(&map);
    }
}
