use std::collections::{BinaryHeap, HashMap, HashSet};

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::new(input);
    let ans = map.solve(1, 3);
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::new(input);
    let ans = map.solve(4, 10);
    Some(ans)
}

struct Map {
    map: HashMap<[isize; 2], i64>,
    m: isize,
    n: isize,
}
impl Map {
    fn new(input: &str) -> Self {
        let mut map = HashMap::new();
        for (i, row) in input.lines().enumerate() {
            for (j, c) in row.chars().enumerate() {
                map.insert([i as isize, j as isize], c.to_digit(10).unwrap() as i64);
            }
        }
        Self {
            map,
            m: input.lines().count() as isize,
            n: input.lines().next().unwrap().chars().count() as isize,
        }
    }
    fn solve(&mut self, min_same_dir: u8, max_same_dir: u8) -> u32 {
        let mut heap: BinaryHeap<(i64, [isize; 2], Direction, u8)> =
            BinaryHeap::from([(0, [0, 0], Direction::None, 1)]);
        let mut visited = HashSet::new();
        while let Some((loss, loc, pre_dir, cnt_dir)) = heap.pop() {
            if cnt_dir >= min_same_dir && loc == [self.m - 1, self.n - 1] {
                return -loss as u32;
            }

            let record = (loc, pre_dir, cnt_dir);
            if visited.contains(&record) {
                continue;
            }
            visited.insert(record);

            let mut next_dirs = HashSet::from([
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]);
            next_dirs.remove(&pre_dir.get_opposite());
            if cnt_dir >= max_same_dir {
                next_dirs.remove(&pre_dir);
            }
            if pre_dir != Direction::None && cnt_dir < min_same_dir {
                next_dirs.retain(|&dir| dir == pre_dir);
            }
            for dir in next_dirs {
                let cnt_dir = if dir == pre_dir { cnt_dir + 1 } else { 1 };
                let next_loc = Self::get_next_pos(loc, &dir);
                if let Some(next_loss) = self.get_val(&next_loc) {
                    if visited.contains(&(next_loc, dir, cnt_dir)) {
                        continue;
                    }
                    heap.push((loss - next_loss, next_loc, dir, cnt_dir));
                }
            }
        }
        0
    }
    fn get_next_pos(loc: [isize; 2], direction: &Direction) -> [isize; 2] {
        let [di, dj] = direction.get_delta();
        [loc[0] + di, loc[1] + dj]
    }
    fn get_val(&self, loc: &[isize; 2]) -> Option<i64> {
        Some(*self.map.get(loc)?)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
impl Direction {
    fn get_delta(&self) -> [isize; 2] {
        match self {
            Self::Up => [-1, 0],
            Self::Down => [1, 0],
            Self::Left => [0, -1],
            Self::Right => [0, 1],
            Self::None => [0, 0],
        }
    }
    fn get_opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
