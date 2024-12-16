use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

advent_of_code::solution!(16);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Pos(isize, isize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Action {
    pos: Pos,
    dir: usize,
}

struct Grid {
    grid: HashSet<Pos>,
    s: Pos,
    e: Pos,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let mut grid = HashSet::new();
        let mut s = Pos(0, 0);
        let mut e = Pos(0, 0);

        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    continue;
                }

                let pos = Pos(i as isize, j as isize);
                grid.insert(pos);

                if c == 'S' {
                    s = pos;
                } else if c == 'E' {
                    e = pos;
                }
            }
        }

        Grid { grid, s, e }
    }

    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

    fn dijkstra(&self) -> (u32, u32) {
        let mut heap: BinaryHeap<(Reverse<u32>, Action, Option<Action>)> = BinaryHeap::from([(
            Reverse(0),
            Action {
                pos: self.s,
                dir: 0,
            },
            None,
        )]);
        let mut min_end_score = u32::MAX;
        let mut min_scores: HashMap<Action, u32> = HashMap::new();
        let mut min_parents: HashMap<Action, HashSet<Action>> = HashMap::new();

        while let Some((Reverse(score), action, old_action)) = heap.pop() {
            if action.pos == self.e {
                if score > min_end_score {
                    break;
                }
                min_end_score = score;
            }

            if let Some(&min_score) = min_scores.get(&action) {
                if min_score == score {
                    if let Some(old_action) = old_action {
                        min_parents.entry(action).or_default().insert(old_action);
                    }
                }
                continue;
            } else {
                if let Some(old_action) = old_action {
                    min_parents.entry(action).or_default().insert(old_action);
                }
                min_scores.insert(action, score);
            }

            for (new_dir, added_score) in [
                (action.dir, 1),
                ((action.dir + 1) % 4, 1001),
                (action.dir.checked_sub(1).unwrap_or(3), 1001),
            ] {
                let delta = Grid::DIRECTIONS[new_dir];
                let new_action = Action {
                    pos: Pos(action.pos.0 + delta.0, action.pos.1 + delta.1),
                    dir: new_dir,
                };

                if self.grid.contains(&new_action.pos) {
                    heap.push((Reverse(score + added_score), new_action, Some(action)));
                }
            }
        }

        (min_end_score, Grid::get_path_nodes_cnt(self.e, min_parents))
    }

    fn get_path_nodes_cnt(end_pos: Pos, min_parents: HashMap<Action, HashSet<Action>>) -> u32 {
        let mut path_nodes: HashSet<Pos> = HashSet::new();

        let mut q = VecDeque::new();
        for action in min_parents.keys() {
            if action.pos == end_pos {
                q.push_back(action);
            }
        }

        while let Some(action) = q.pop_front() {
            path_nodes.insert(action.pos);
            if let Some(parents) = min_parents.get(action) {
                for parent in parents {
                    q.push_back(parent);
                }
            }
        }

        path_nodes.len() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);

    let (min_end_score, _) = grid.dijkstra();

    Some(min_end_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);

    let (_, path_nodes_cnt) = grid.dijkstra();

    Some(path_nodes_cnt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
