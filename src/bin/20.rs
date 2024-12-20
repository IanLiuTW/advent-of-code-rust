use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let distance_from_start = grid.get_distance_from(grid.start);
    let distance_from_end = grid.get_distance_from(grid.end);
    let standard_dis = *distance_from_start.get(&grid.end).unwrap();

    let cnter =
        grid.get_result_after_cheating(&distance_from_start, &distance_from_end, standard_dis, 2);
    let ans = cnter
        .iter()
        .filter_map(|(k, v)| (*k >= 100).then_some(v))
        .sum::<u32>();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let distance_from_start = grid.get_distance_from(grid.start);
    let distance_from_end = grid.get_distance_from(grid.end);
    let standard_dis = *distance_from_start.get(&grid.end).unwrap();

    let cnter =
        grid.get_result_after_cheating(&distance_from_start, &distance_from_end, standard_dis, 20);
    let ans = cnter
        .iter()
        .filter_map(|(k, v)| (*k >= 100).then_some(v))
        .sum::<u32>();

    Some(ans)
}

type Pos = (isize, isize);

struct Grid {
    track: HashSet<Pos>,
    start: Pos,
    end: Pos,
    m: isize,
    n: isize,
}

impl Grid {
    const DIRECTIONS: [Pos; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn get_distance_from(&self, start: Pos) -> HashMap<(isize, isize), i32> {
        let mut distance = HashMap::new();

        let mut deq = VecDeque::from([(start, 0)]);
        let mut visited = HashSet::from([start]);

        while let Some((pos, dis)) = deq.pop_front() {
            distance.insert(pos, dis);

            for delta in Grid::DIRECTIONS {
                let new_pos = (pos.0 + delta.0, pos.1 + delta.1);
                if self.is_reachable(new_pos) && !visited.contains(&new_pos) {
                    deq.push_back((new_pos, dis + 1));
                    visited.insert(new_pos);
                }
            }
        }

        distance
    }

    fn get_result_after_cheating(
        &self,
        distance_from_start: &HashMap<(isize, isize), i32>,
        distance_from_end: &HashMap<(isize, isize), i32>,
        standard_dis: i32,
        max_steps: i32,
    ) -> HashMap<i32, u32> {
        let mut cnter: HashMap<i32, u32> = HashMap::new();

        for (pos, dis_to_start) in distance_from_start {
            for (new_pos, steps) in self.get_possible_pos_after_cheating(*pos, max_steps) {
                if let Some(dis_to_end) = distance_from_end.get(&new_pos) {
                    let new_dis = dis_to_start + steps + dis_to_end;
                    if new_dis < standard_dis {
                        *cnter.entry(standard_dis - new_dis).or_default() += 1;
                    }
                }
            }
        }

        cnter
    }

    fn get_possible_pos_after_cheating(
        &self,
        pos: Pos,
        max_steps: i32,
    ) -> HashSet<((isize, isize), i32)> {
        let mut possible_shortcut_pos = HashSet::new();
        let mut visited = HashSet::new();

        let mut q = VecDeque::from([(pos, 0)]);
        while let Some((pos, steps)) = q.pop_front() {
            let new_steps = steps + 1;
            if new_steps > max_steps {
                continue;
            }

            for delta in Grid::DIRECTIONS {
                let new_pos = (pos.0 + delta.0, pos.1 + delta.1);
                if !visited.contains(&new_pos)
                    && (0..self.m).contains(&new_pos.0)
                    && (0..self.n).contains(&new_pos.1)
                {
                    visited.insert(new_pos);
                    q.push_back((new_pos, new_steps));
                    if new_steps >= 2 {
                        possible_shortcut_pos.insert((new_pos, new_steps));
                    }
                }
            }
        }

        possible_shortcut_pos
    }

    fn is_reachable(&self, new_pos: Pos) -> bool {
        self.track.contains(&new_pos)
    }
}

fn parse_input(input: &str) -> Grid {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut track = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    track.insert((i as isize, j as isize));
                }
                'S' => {
                    track.insert((i as isize, j as isize));
                    start = (i as isize, j as isize);
                }
                'E' => {
                    track.insert((i as isize, j as isize));
                    end = (i as isize, j as isize);
                }
                _ => (),
            };
        }
    }

    let m = input.len() as isize;
    let n = input.lines().next().unwrap().len() as isize;

    Grid {
        track,
        start,
        end,
        m,
        n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let grid = parse_input(&advent_of_code::template::read_file("examples", DAY));

        let distance_from_start = grid.get_distance_from(grid.start);
        let distance_from_end = grid.get_distance_from(grid.end);
        let standard_dis = *distance_from_start.get(&grid.end).unwrap();

        let cnter = grid.get_result_after_cheating(
            &distance_from_start,
            &distance_from_end,
            standard_dis,
            2,
        );
        assert_eq!(
            cnter,
            HashMap::from([
                (38, 1),
                (8, 4),
                (6, 2),
                (10, 2),
                (20, 1),
                (64, 1),
                (4, 14),
                (2, 14),
                (12, 3),
                (36, 1),
                (40, 1),
            ])
        );
    }

    #[test]
    fn test_part_two() {
        let grid = parse_input(&advent_of_code::template::read_file("examples", DAY));

        let distance_from_start = grid.get_distance_from(grid.start);
        let distance_from_end = grid.get_distance_from(grid.end);
        let standard_dis = *distance_from_start.get(&grid.end).unwrap();

        let cnter = grid.get_result_after_cheating(
            &distance_from_start,
            &distance_from_end,
            standard_dis,
            20,
        );
        dbg!(&cnter);
    }
}
