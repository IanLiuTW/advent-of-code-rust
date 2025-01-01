use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let ans = grid.count_all_visibles();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let ans = grid.get_best_visibility();

    Some(ans)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(usize, usize);

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u32>>,
    m: usize,
    n: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec();
        let (m, n) = (grid.len(), grid[0].len());

        Self { grid, m, n }
    }

    fn get_val(&self, pos: &Pos) -> u32 {
        self.grid[pos.0][pos.1]
    }

    fn get_new_pos(&self, pos: Pos, delta: (isize, isize)) -> Option<Pos> {
        let i = pos.0.checked_add_signed(delta.0)?;
        let j = pos.1.checked_add_signed(delta.1)?;

        (self.m > i && self.n > j).then_some(Pos(i, j))
    }

    fn _get_visible_pos_in_a_direction(&self, mut pos: Pos, delta: (isize, isize)) -> Vec<Pos> {
        let mut visible_pos = vec![pos];

        while let Some(new_pos) = self.get_new_pos(pos, delta) {
            pos = new_pos;

            if let Some(last) = visible_pos.last() {
                if self.get_val(last) >= self.get_val(&new_pos) {
                    continue;
                }
            }
            visible_pos.push(new_pos);
        }

        visible_pos
    }

    fn count_all_visibles(&self) -> u32 {
        let mut visible_pos = HashSet::new();

        for i in 0..self.m {
            visible_pos.extend(self._get_visible_pos_in_a_direction(Pos(i, 0), (0, 1)));
            visible_pos.extend(self._get_visible_pos_in_a_direction(Pos(i, self.n - 1), (0, -1)));
        }
        for j in 0..self.n {
            visible_pos.extend(self._get_visible_pos_in_a_direction(Pos(0, j), (1, 0)));
            visible_pos.extend(self._get_visible_pos_in_a_direction(Pos(self.m - 1, j), (-1, 0)));
        }

        visible_pos.len() as u32
    }

    fn _update_visibility_in_a_direction(
        &self,
        mut pos: Pos,
        delta: (isize, isize),
        visibility: &mut Vec<Vec<u32>>,
    ) {
        let mut stack = vec![(u32::MAX, 0)];
        let mut i = 0;

        loop {
            let height = self.get_val(&pos);
            while stack.last().unwrap().0 < height {
                stack.pop();
            }

            visibility[pos.0][pos.1] *= i - stack.last().unwrap().1;

            stack.push((height, i));

            if let Some(new_pos) = self.get_new_pos(pos, delta) {
                pos = new_pos;
                i += 1;
            } else {
                break;
            }
        }
    }

    fn get_best_visibility(&self) -> u32 {
        let mut visibility = vec![vec![1; self.n]; self.m];

        for i in 0..self.m {
            self._update_visibility_in_a_direction(Pos(i, 0), (0, 1), &mut visibility);
            self._update_visibility_in_a_direction(Pos(i, self.n - 1), (0, -1), &mut visibility);
        }
        for j in 0..self.n {
            self._update_visibility_in_a_direction(Pos(0, j), (1, 0), &mut visibility);
            self._update_visibility_in_a_direction(Pos(self.m - 1, j), (-1, 0), &mut visibility);
        }

        *visibility.iter().flatten().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
