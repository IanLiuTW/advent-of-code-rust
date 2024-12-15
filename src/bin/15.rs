use core::str;

use itertools::Itertools;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, instructions) = parse_intput(input, 1);
    for direction in instructions {
        grid.move_robot(direction);
    }

    Some(grid.get_gps_sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut grid, instructions) = parse_intput(input, 2);
    for direction in instructions {
        grid.move_robot(direction);
    }

    Some(grid.get_gps_sum())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ObjectType {
    Wall,
    Empty,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
}

impl ObjectType {
    fn to_char(self) -> char {
        match self {
            ObjectType::Wall => '#',
            ObjectType::Empty => ' ',
            ObjectType::Box => 'O',
            ObjectType::BoxLeft => '[',
            ObjectType::BoxRight => ']',
            ObjectType::Robot => '@',
        }
    }
}

struct Grid {
    grid: Vec<Vec<ObjectType>>,
    m: usize,
    n: usize,
    ri: usize,
    rj: usize,
}

impl Grid {
    fn new(grid_lines: Vec<&str>, warehouse_number: u32) -> Self {
        let grid_builder = match warehouse_number {
            1 => Grid::grid_builder_warehouse_1,
            2 => Grid::grid_builder_warehouse_2,
            _ => unreachable!(),
        };
        let grid = grid_builder(grid_lines);

        let (m, n) = (grid.len(), grid[0].len());
        for (ri, rj) in (0..m).cartesian_product(0..n) {
            if grid[ri][rj] == ObjectType::Robot {
                return Grid { grid, m, n, ri, rj };
            }
        }
        unreachable!()
    }

    fn grid_builder_warehouse_1(grid_lines: Vec<&str>) -> Vec<Vec<ObjectType>> {
        let mut grid: Vec<Vec<ObjectType>> = vec![];

        for line in grid_lines {
            let mut row = vec![];
            for c in line.chars() {
                row.push(match c {
                    '#' => ObjectType::Wall,
                    'O' => ObjectType::Box,
                    '@' => ObjectType::Robot,
                    _ => ObjectType::Empty,
                })
            }
            grid.push(row);
        }

        grid
    }

    fn grid_builder_warehouse_2(grid_lines: Vec<&str>) -> Vec<Vec<ObjectType>> {
        let mut grid: Vec<Vec<ObjectType>> = vec![];

        for line in grid_lines {
            let mut row = vec![];
            for c in line.chars() {
                match c {
                    '#' => row.extend(vec![ObjectType::Wall, ObjectType::Wall]),
                    'O' => row.extend(vec![ObjectType::BoxLeft, ObjectType::BoxRight]),
                    '@' => row.extend(vec![ObjectType::Robot, ObjectType::Empty]),
                    _ => row.extend(vec![ObjectType::Empty, ObjectType::Empty]),
                }
            }
            grid.push(row);
        }

        grid
    }

    fn get_delta(direction: char) -> (isize, isize) {
        match direction {
            '^' => (-1, 0),
            'v' => (1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            _ => unreachable!(),
        }
    }

    fn get_next_pos(pos: (usize, usize), delta: (isize, isize)) -> (usize, usize) {
        let ni = pos.0 as isize + delta.0;
        let nj = pos.1 as isize + delta.1;
        (ni as usize, nj as usize)
    }

    fn move_robot(&mut self, direction: char) {
        let pos = (self.ri, self.rj);
        let delta = Grid::get_delta(direction);

        if self.is_path_clear_to_move(pos, delta) {
            self.move_objects_in_path(pos, delta);

            let next_pos = Grid::get_next_pos(pos, delta);
            self.ri = next_pos.0;
            self.rj = next_pos.1;
        }
    }

    fn is_path_clear_to_move(&mut self, pos: (usize, usize), delta: (isize, isize)) -> bool {
        match self.grid[pos.0][pos.1] {
            ObjectType::Wall => false,
            ObjectType::Empty => true,
            ObjectType::Box | ObjectType::Robot => {
                let next_pos = Grid::get_next_pos(pos, delta);
                self.is_path_clear_to_move(next_pos, delta)
            }
            ObjectType::BoxLeft | ObjectType::BoxRight => {
                if [(0, 1), (0, -1)].contains(&delta) {
                    let next_pos = Grid::get_next_pos(pos, delta);
                    self.is_path_clear_to_move(next_pos, delta)
                } else {
                    let (left_pos, right_pos) = if self.grid[pos.0][pos.1] == ObjectType::BoxLeft {
                        (pos, Grid::get_next_pos(pos, (0, 1)))
                    } else {
                        (Grid::get_next_pos(pos, (0, -1)), pos)
                    };

                    let next_left_pos = Grid::get_next_pos(left_pos, delta);
                    let next_right_pos = Grid::get_next_pos(right_pos, delta);

                    self.is_path_clear_to_move(next_left_pos, delta)
                        && self.is_path_clear_to_move(next_right_pos, delta)
                }
            }
        }
    }

    fn move_objects_in_path(&mut self, pos: (usize, usize), delta: (isize, isize)) {
        match self.grid[pos.0][pos.1] {
            ObjectType::Wall => unreachable!(),
            ObjectType::Empty => (),
            ObjectType::Box | ObjectType::Robot => {
                let next_pos = Grid::get_next_pos(pos, delta);
                self.move_objects_in_path(next_pos, delta);

                self.grid[next_pos.0][next_pos.1] = self.grid[pos.0][pos.1];
                self.grid[pos.0][pos.1] = ObjectType::Empty;
            }
            ObjectType::BoxLeft | ObjectType::BoxRight => {
                if [(0, 1), (0, -1)].contains(&delta) {
                    let next_pos = Grid::get_next_pos(pos, delta);
                    self.move_objects_in_path(next_pos, delta);

                    self.grid[next_pos.0][next_pos.1] = self.grid[pos.0][pos.1];
                    self.grid[pos.0][pos.1] = ObjectType::Empty;
                } else {
                    let (left_pos, right_pos) = if self.grid[pos.0][pos.1] == ObjectType::BoxLeft {
                        (pos, Grid::get_next_pos(pos, (0, 1)))
                    } else {
                        (Grid::get_next_pos(pos, (0, -1)), pos)
                    };

                    let next_left_pos = Grid::get_next_pos(left_pos, delta);
                    let next_right_pos = Grid::get_next_pos(right_pos, delta);
                    self.move_objects_in_path(next_left_pos, delta);
                    self.move_objects_in_path(next_right_pos, delta);

                    self.grid[next_left_pos.0][next_left_pos.1] = self.grid[left_pos.0][left_pos.1];
                    self.grid[left_pos.0][left_pos.1] = ObjectType::Empty;
                    self.grid[next_right_pos.0][next_right_pos.1] =
                        self.grid[right_pos.0][right_pos.1];
                    self.grid[right_pos.0][right_pos.1] = ObjectType::Empty;
                }
            }
        }
    }

    fn get_gps_sum(&self) -> u32 {
        let mut ans = 0;
        for (i, j) in (0..self.m).cartesian_product(0..self.n) {
            if self.grid[i][j] == ObjectType::Box || self.grid[i][j] == ObjectType::BoxLeft {
                ans += 100 * i + j;
            }
        }
        ans as u32
    }
    //
    // fn get_gps_sum_warehouse_2(&self) -> u32 {
    //     let mut ans = 0;
    //     for (i, j) in (0..self.m).cartesian_product(0..self.n) {
    //         if self.grid[i][j] == ObjectType::BoxLeft {
    //             let left_pos = (i, j);
    //             let right_pos = Grid::get_next_pos(left_pos, (0, 1));
    //
    //             let gps = [
    //                 left_pos.0 * 100 + left_pos.1,
    //                 // (self.m - left_pos.0 - 1) * 100 + left_pos.1,
    //                 // right_pos.0 * 100 + (self.n - right_pos.1- 1),
    //                 // (self.m - right_pos.0 - 1) * 100 + (self.n - right_pos.1 - 1),
    //             ];
    //
    //             dbg!(&gps);
    //             ans += gps.iter().min().unwrap();
    //         }
    //     }
    //     ans as u32
    // }

    #[allow(dead_code)]
    fn draw_grid(&self) {
        for row in &self.grid {
            println!(
                "{}",
                row.iter().map(|c| { c.to_char() }).collect::<String>()
            );
        }
        println!("{} {}", self.ri, self.rj);
    }
}

fn parse_intput(input: &str, warehose_number: u32) -> (Grid, Vec<char>) {
    let mut input = input.lines();

    let mut grid_lines = vec![];
    loop {
        let line = input.next().unwrap();
        if line.is_empty() {
            break;
        }
        grid_lines.push(line);
    }
    let grid = Grid::new(grid_lines, warehose_number);

    let mut instructions = vec![];
    for line in input {
        for c in line.chars() {
            instructions.push(c);
        }
    }

    (grid, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let (mut grid, instructions) =
            parse_intput(&advent_of_code::template::read_file("examples", DAY), 1);
        // grid.draw_grid();
        for direction in instructions {
            grid.move_robot(direction);
            // grid.draw_grid();
        }
        let gps_sum = grid.get_gps_sum();
        assert_eq!(gps_sum, 10092);
    }

    #[test]
    fn test_part_two() {
        let (mut grid, instructions) =
            parse_intput(&advent_of_code::template::read_file("examples", DAY), 2);
        grid.draw_grid();
        for direction in instructions {
            grid.move_robot(direction);
            grid.draw_grid();
        }
        let gps_sum = grid.get_gps_sum();
        assert_eq!(gps_sum, 9021);
    }
}
