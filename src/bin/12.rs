use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::new(input);

    Some(grid.bfs())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::new(input);

    Some(grid.bfs2())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(usize, usize);

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u32>>,
    m: usize,
    n: usize,
    start: Pos,
    end: Pos,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut start = Pos(0, 0);
        let mut end = Pos(0, 0);

        let mut grid = vec![];
        for (i, line) in input.lines().enumerate() {
            let mut row = vec![];
            for (j, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        row.push('a' as u32);
                        start = Pos(i, j);
                    }
                    'E' => {
                        row.push('z' as u32);
                        end = Pos(i, j);
                    }
                    _ => row.push(c as u32),
                }
            }
            grid.push(row);
        }

        let [m, n] = [grid.len(), grid[0].len()];

        Grid {
            grid,
            m,
            n,
            start,
            end,
        }
    }

    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn bfs(&mut self) -> u32 {
        let mut q = VecDeque::from([(self.start, 0)]);
        let mut visited = HashSet::from([self.start]);
        let condition: Box<dyn Fn(u32, u32) -> bool> = Box::new(|a: u32, b: u32| a + 1 >= b);

        while let Some((pos, steps)) = q.pop_front() {
            if pos == self.end {
                return steps;
            }

            for delta in Grid::DIRECTIONS {
                if let Some(new_pos) = self.get_new_pos(&pos, &delta, &condition, &mut visited) {
                    q.push_back((new_pos, steps + 1));
                }
            }
        }

        u32::MAX
    }

    fn bfs2(&mut self) -> u32 {
        let mut q = VecDeque::from([(self.end, 0)]);
        let mut visited = HashSet::from([self.end]);
        let condition: Box<dyn Fn(u32, u32) -> bool> = Box::new(|a: u32, b: u32| a <= b + 1);

        while let Some((pos, steps)) = q.pop_front() {
            if self.get_val(&pos) == 97 {
                return steps;
            }

            for delta in Grid::DIRECTIONS {
                if let Some(new_pos) = self.get_new_pos(&pos, &delta, &condition, &mut visited) {
                    q.push_back((new_pos, steps + 1));
                }
            }
        }

        u32::MAX
    }

    fn get_new_pos(
        &mut self,
        pos: &Pos,
        delta: &(isize, isize),
        condition: &dyn Fn(u32, u32) -> bool,
        visited: &mut HashSet<Pos>,
    ) -> Option<Pos> {
        let i = pos.0.checked_add_signed(delta.0)?;
        let j = pos.1.checked_add_signed(delta.1)?;

        if i >= self.m || j >= self.n {
            return None;
        }

        let new_pos = Pos(i, j);
        if !condition(self.get_val(pos), self.get_val(&new_pos)) {
            return None;
        }

        if !visited.insert(new_pos) {
            return None;
        }

        Some(new_pos)
    }

    fn get_val(&self, pos: &Pos) -> u32 {
        self.grid[pos.0][pos.1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(29));
    }
}
