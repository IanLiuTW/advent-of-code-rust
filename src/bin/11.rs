advent_of_code::solution!(11);

use bisection::bisect_left;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 1)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 999999)
}

fn solve(input: &str, expansion_coe: usize) -> Option<u64> {
    let map = Map::new(input);
    let coord = map.get_expanded_coordinates(expansion_coe);

    let mut ans = 0;
    for v in coord.iter().combinations(2) {
        ans += get_l1_dis(v[0], v[1]);
    }
    Some(ans)
}

struct Map {
    map: Vec<Vec<char>>,
}
impl Map {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
        Self { map }
    }

    fn get_expanded_coordinates(&self, coe: usize) -> Vec<[usize; 2]> {
        let mut coord = self.get_coordinates();
        let empty_rows = self.get_empty_rows();
        let empty_cols = self.get_empty_cols();
        for [i, j] in coord.iter_mut() {
            *i += Map::get_expanded_dis(i, &empty_rows) * coe;
            *j += Map::get_expanded_dis(j, &empty_cols) * coe;
        }
        coord
    }

    fn get_coordinates(&self) -> Vec<[usize; 2]> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, chr)| if *chr == '#' { Some([i, j]) } else { None })
            })
            .collect()
    }

    fn get_empty_rows(&self) -> Vec<usize> {
        self.map
            .iter()
            .enumerate()
            .filter_map(|(i, row)| if row.contains(&'#') { None } else { Some(i) })
            .collect()
    }

    fn get_empty_cols(&self) -> Vec<usize> {
        (0..self.map[0].len())
            .filter(|j| {
                let col: Vec<char> = self.map.iter().map(|row| row[*j]).collect();
                !col.contains(&'#')
            })
            .collect()
    }

    fn get_expanded_dis(idx: &usize, empty_idx: &[usize]) -> usize {
        bisect_left(empty_idx, idx)
    }
}

fn get_l1_dis(x: &[usize; 2], y: &[usize; 2]) -> u64 {
    fn get_abs(i1: usize, i2: usize) -> usize {
        usize::max(i1, i2) - usize::min(i1, i2)
    }
    (get_abs(x[0], y[0]) + get_abs(x[1], y[1])) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
