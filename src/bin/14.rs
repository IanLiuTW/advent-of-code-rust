use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    i: usize,
    j: usize,
    di: isize,
    dj: isize,
}

struct Grid {
    m: usize,
    n: usize,
    robots: Vec<Robot>,
    mid_m: usize,
    mid_n: usize,
}

impl Grid {
    fn new(m: usize, n: usize, robots: Vec<Robot>) -> Self {
        Grid {
            m,
            n,
            robots,
            mid_m: m / 2,
            mid_n: n / 2,
        }
    }

    fn move_robots(&mut self) {
        for robot in &mut self.robots {
            robot.i = ((robot.i as isize + robot.di).rem_euclid(self.m as isize)) as usize;
            robot.j = ((robot.j as isize + robot.dj).rem_euclid(self.n as isize)) as usize;
        }
    }

    fn find_safety_factor(&self) -> u32 {
        let mut quadrant: HashMap<(bool, bool), u32> = HashMap::new();

        for robot in &self.robots {
            if robot.i == self.mid_m || robot.j == self.mid_n {
                continue;
            }
            *quadrant
                .entry((robot.i > self.mid_m, robot.j > self.mid_n))
                .or_default() += 1;
        }

        quadrant.values().product()
    }

    fn has_a_tree(&self) -> bool {
        let mut rows: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut cols: HashMap<usize, Vec<usize>> = HashMap::new();

        for robot in &self.robots {
            rows.entry(robot.i).or_default().push(robot.j);
            cols.entry(robot.j).or_default().push(robot.i);
        }

        let mut crowded_rows = 0;
        for row in rows.values() {
            if row.len() >= 30 {
                crowded_rows += 1;
            }
        }
        let mut crowded_cols = 0;
        for col in cols.values() {
            if col.len() >= 30 {
                crowded_cols += 1;
            }
        }

        crowded_rows >= 2 && crowded_cols >= 2 // has a frame
    }

    fn draw_grid(&self) {
        let mut canvas = vec![vec!['.'; self.n]; self.m];
        for robot in &self.robots {
            canvas[robot.i][robot.j] = '󱚣';
        }
        for row in canvas {
            println!("{}", row.iter().collect::<String>());
        }
    }

    #[allow(dead_code)]
    fn dbg_robots(&self) {
        for robot in &self.robots {
            dbg!(robot);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let robots = parse_input(input);
    let mut grid = Grid::new(103, 101, robots);

    for _ in 0..100 {
        grid.move_robots();
    }

    Some(grid.find_safety_factor())
}

pub fn part_two(input: &str) -> Option<u32> {
    let robots = parse_input(input);
    let mut grid = Grid::new(103, 101, robots);

    for i in 1..=10000 {
        grid.move_robots();
        if grid.has_a_tree() {
            grid.draw_grid();
            return Some(i as u32);
        }
    }

    None
}

fn parse_input(input: &str) -> Vec<Robot> {
    let line_re = Regex::new(r"p\=(.*),(.*)\sv\=(.*),(.*)").unwrap();
    let mut parsed_input = vec![];

    input.lines().for_each(|line| {
        let cap = line_re.captures(line).unwrap();
        parsed_input.push(Robot {
            i: cap[2].parse().unwrap(),
            j: cap[1].parse().unwrap(),
            di: cap[4].parse().unwrap(),
            dj: cap[3].parse().unwrap(),
        })
    });

    parsed_input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let robots = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let mut grid = Grid::new(7, 11, robots);
        for _ in 0..100 {
            grid.move_robots();
            grid.draw_grid();
        }
        assert_eq!(grid.find_safety_factor(), 12);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    }
}
