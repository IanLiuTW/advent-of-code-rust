use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

advent_of_code::solution!(18);

struct Grid {
    m: isize,
    n: isize,
    obstacles: HashSet<(isize, isize)>,
    start: (isize, isize),
    end: (isize, isize),
}

impl Grid {
    fn new(m: isize, n: isize, input: &Vec<(isize, isize)>, take_cnt: usize) -> Self {
        let obstacles = input.iter().take(take_cnt).cloned().collect();
        Self {
            m,
            n,
            obstacles,
            start: (0, 0),
            end: (m - 1, n - 1),
        }
    }

    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn dijkstra(&self) -> Option<u32> {
        let mut visited = HashSet::new();
        let mut heap = BinaryHeap::from([(Reverse(0), self.start)]);

        while let Some((Reverse(steps), pos)) = heap.pop() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            if pos == self.end {
                return Some(steps);
            }

            for dir in Grid::DIRECTIONS {
                let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                if !visited.contains(&new_pos) && self.is_reachable(new_pos) {
                    heap.push((Reverse(steps + 1), new_pos));
                }
            }
        }

        None
    }

    fn is_reachable(&self, new_pos: (isize, isize)) -> bool {
        self.is_inboud(new_pos.0, new_pos.1) && !self.obstacles.contains(&new_pos)
    }

    fn is_inboud(&self, ni: isize, nj: isize) -> bool {
        (0..self.m).contains(&ni) && (0..self.n).contains(&nj)
    }
}

fn parse_input(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split(',');
            (
                line.next().unwrap().parse().unwrap(),
                line.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(isize, isize)>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);

    let grid_size = 71;
    let take_cnt = 1024;
    let grid = Grid::new(grid_size, grid_size, &input, take_cnt);

    grid.dijkstra()
}

pub fn part_two(input: &str) -> Option<String> {
    let input = parse_input(input);

    let grid_size = 71;

    let mut lo = 0;
    let mut hi = input.len() - 1;
    while lo < hi {
        let mid = (lo + hi) / 2;
        let grid = Grid::new(grid_size, grid_size, &input, mid);

        if grid.dijkstra().is_none() {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    let pair = input[lo-1];

    Some(format!("{},{}", pair.0, pair.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = parse_input(&advent_of_code::template::read_file("examples", DAY));

        let grid_size = 7;
        let take_cnt = 12;
        let grid = Grid::new(grid_size, grid_size, &input, take_cnt);

        let result = grid.dijkstra();
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(&advent_of_code::template::read_file("examples", DAY));

        let grid_size = 7;

        let mut lo = 0;
        let mut hi = input.len() - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            let grid = Grid::new(grid_size, grid_size, &input, mid);

            if grid.dijkstra().is_none() {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        assert_eq!(input[lo - 1], (6, 1));
    }
}
