use itertools::Itertools;

advent_of_code::solution!(4);

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const EIGHT_DIRECTIONS: [(isize, isize); 9] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, 0),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];
const X_PATTERN: [[(isize, isize); 2]; 2] = [[(-1, -1), (1, 1)], [(-1, 1), (1, -1)]];

pub fn part_one(input: &str) -> Option<u32> {
    let grid = convert_input_to_grid(input);
    let bounds = (grid.len(), grid[0].len());

    let mut ans = 0;
    for pos in (0..bounds.0).cartesian_product(0..bounds.1) {
        for delta in EIGHT_DIRECTIONS {
            ans += dfs(&grid, pos, delta, bounds, 0);
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = convert_input_to_grid(input);
    let bounds = (grid.len(), grid[0].len());

    let mut ans = 0;
    for pos in (0..bounds.0).cartesian_product(0..bounds.1) {
        if has_x_pattern(&grid, pos, bounds) {
            ans += 1;
        }
    }

    Some(ans)
}

fn dfs(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    delta: (isize, isize),
    bounds: (usize, usize),
    mut idx: usize,
) -> u32 {
    let mut ans = 0;

    let char = XMAS[idx];
    if grid[pos.0][pos.1] != char {
        return 0;
    }

    idx += 1;
    if idx == XMAS.len() {
        return 1;
    }

    if let Some(new_pos) = get_next_move(pos, delta, bounds) {
        ans += dfs(grid, new_pos, delta, bounds, idx);
    }

    ans
}

fn has_x_pattern(grid: &Vec<Vec<char>>, pos: (usize, usize), bounds: (usize, usize)) -> bool {
    if grid[pos.0][pos.1] != 'A' {
        return false;
    }
    for pattern in X_PATTERN {
        let mut chars = vec![];
        for delta in pattern {
            if let Some((ni, nj)) = get_next_move(pos, delta, bounds) {
                chars.push(grid[ni][nj]);
            } else {
                return false;
            }
        }
        if !(chars.contains(&'M') && chars.contains(&'S')) {
            return false;
        }
    }
    true
}

fn convert_input_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_next_move(
    pos: (usize, usize),
    delta: (isize, isize),
    bounds: (usize, usize),
) -> Option<(usize, usize)> {
    let ni = pos.0 as isize + delta.0;
    let nj = pos.1 as isize + delta.1;
    if (0..bounds.0 as isize).contains(&ni) && (0..bounds.1 as isize).contains(&nj) {
        Some((ni as usize, nj as usize))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
