use anyhow::{Context, Result, anyhow};
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

advent_of_code::solution!(10);

#[derive(Debug, Clone)]
struct Manual {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl FromStr for Manual {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        let lights = tokens
            .next()
            .context("Missing lights section")?
            .trim_matches(['[', ']'])
            .chars()
            .map(|c| match c {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err(anyhow!("Invalid light: {}", c)),
            })
            .collect::<Result<Vec<_>>>()?;

        let joltage = tokens
            .next_back()
            .context("Missing joltage section")?
            .trim_matches(['{', '}'])
            .split(',')
            .map(|n| n.parse().context("Invalid joltage number"))
            .collect::<Result<Vec<usize>>>()?;

        let buttons = tokens
            .map(|t| {
                t.trim_matches(['(', ')'])
                    .split(',')
                    .map(|n| n.parse().context("Invalid button number"))
                    .collect::<Result<Vec<usize>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Manual {
            lights,
            buttons,
            joltage,
        })
    }
}

impl Manual {
    fn solve_for_lights(&self) -> Option<u64> {
        let target_mask = self.bitmask_lights();

        if target_mask == 0 {
            return Some(0);
        }

        let button_masks = self.bitmask_buttons();

        let mut queue = VecDeque::from([(0u64, 0u64)]);
        let mut visited = HashSet::from([0]);

        while let Some((current_mask, steps)) = queue.pop_front() {
            for &b_mask in &button_masks {
                let next_mask = current_mask ^ b_mask;

                if next_mask == target_mask {
                    return Some(steps + 1);
                }

                if visited.insert(next_mask) {
                    queue.push_back((next_mask, steps + 1));
                }
            }
        }

        None
    }

    fn bitmask_lights(&self) -> u64 {
        self.lights
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &b)| if b { acc | (1 << i) } else { acc })
    }

    fn bitmask_buttons(&self) -> Vec<u64> {
        self.buttons
            .iter()
            .map(|b| b.iter().fold(0, |acc, &num| acc | (1 << num)))
            .collect()
    }
}

impl Manual {
    pub fn solve_for_joltage(&self) -> Option<usize> {
        let n_eq = self.joltage.len();
        let n_vars = self.buttons.len();

        // Build augmented matrix
        let mut matrix = vec![vec![0.0; n_vars + 1]; n_eq];

        for (col, btn_indices) in self.buttons.iter().enumerate() {
            for &row in btn_indices {
                if row < n_eq {
                    matrix[row][col] = 1.0;
                }
            }
        }

        for (row, &val) in self.joltage.iter().enumerate() {
            matrix[row][n_vars] = val as f64;
        }

        // Gaussian elimination
        let mut pivot_row = 0;
        let mut col_to_pivot_row = vec![None; n_vars];
        let mut free_vars = Vec::new();

        for col in 0..n_vars {
            if pivot_row >= n_eq {
                free_vars.push(col);
                continue;
            }

            // find pivot
            let mut max_row = pivot_row;
            for r in pivot_row + 1..n_eq {
                if matrix[r][col].abs() > matrix[max_row][col].abs() {
                    max_row = r;
                }
            }

            if matrix[max_row][col].abs() < 1e-9 {
                free_vars.push(col);
                continue;
            }

            matrix.swap(pivot_row, max_row);

            // normalize pivot row
            let pivot_val = matrix[pivot_row][col];
            for j in col..=n_vars {
                matrix[pivot_row][j] /= pivot_val;
            }

            // eliminate column
            for r in 0..n_eq {
                if r == pivot_row {
                    continue;
                }
                let factor = matrix[r][col];
                if factor.abs() > 1e-9 {
                    for j in col..=n_vars {
                        matrix[r][j] -= factor * matrix[pivot_row][j];
                    }
                }
            }

            col_to_pivot_row[col] = Some(pivot_row);
            pivot_row += 1;
        }

        // check consistency
        for r in pivot_row..n_eq {
            if matrix[r][n_vars].abs() > 1e-9 {
                return None;
            }
        }

        // Build free var mask
        let mut is_free = vec![false; n_vars];
        for &fv in &free_vars {
            is_free[fv] = true;
        }

        // Build compact pivot equations
        // Each pivot column x[col] = rhs - sum(coeff[k] * x[k])
        #[derive(Clone)]
        struct PivotEq {
            rhs: f64,
            coeff: Vec<(usize, f64)>,
        }

        let mut pivots = Vec::new();
        for col in 0..n_vars {
            if let Some(r) = col_to_pivot_row[col] {
                if is_free[col] {
                    continue;
                }

                let row = &matrix[r];
                let mut coeff = Vec::new();
                for fv in &free_vars {
                    let c = row[*fv];
                    if c.abs() > 1e-12 {
                        coeff.push((*fv, c));
                    }
                }

                let rhs = row[n_vars];
                pivots.push((col, PivotEq { rhs, coeff }));
            }
        }

        // bound for each free var
        let mut bounds = vec![0usize; n_vars];
        for &fv in &free_vars {
            let mut b = usize::MAX;
            for &row in &self.buttons[fv] {
                if row < n_eq && self.joltage[row] < b {
                    b = self.joltage[row];
                }
            }
            bounds[fv] = if b == usize::MAX { 0 } else { b };
        }

        let mut best_sum: Option<usize> = None;
        let mut solution = vec![0.0; n_vars];

        // DFS function
        fn dfs(
            idx: usize,
            free_vars: &[usize],
            bounds: &[usize],
            solution: &mut [f64],
            best_sum: &mut Option<usize>,
            pivots: &[(usize, PivotEq)],
        ) {
            // pruning
            if let Some(best) = best_sum {
                let mut partial = 0usize;
                for i in 0..idx {
                    let fv = free_vars[i];
                    partial += solution[fv] as usize;
                }
                if partial >= *best {
                    return;
                }
            }

            if idx == free_vars.len() {
                let mut sum = 0usize;

                // sum free variables
                for &fv in free_vars {
                    sum += solution[fv] as usize;
                }

                // compute dependent variables
                for (_col, eq) in pivots {
                    let mut val = eq.rhs;
                    for &(fv, c) in &eq.coeff {
                        val -= c * solution[fv];
                    }

                    if val < -0.0005 {
                        return;
                    }
                    let r = val.round();
                    if (val - r).abs() > 0.001 {
                        return;
                    }

                    sum += r as usize;
                }

                match best_sum {
                    None => *best_sum = Some(sum),
                    Some(b) => {
                        if sum < *b {
                            *best_sum = Some(sum);
                        }
                    }
                }
                return;
            }

            let fv = free_vars[idx];
            let bound = bounds[fv];

            for v in 0..=bound {
                solution[fv] = v as f64;
                dfs(idx + 1, free_vars, bounds, solution, best_sum, pivots);
            }
        }

        // kick off search
        dfs(0, &free_vars, &bounds, &mut solution, &mut best_sum, &pivots);

        best_sum
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = input
        .lines()
        .filter_map(|line| Manual::from_str(line).ok())
        .collect::<Vec<Manual>>();

    let result = machines
        .iter()
        .filter_map(|m| m.solve_for_lights())
        .sum::<u64>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = input
        .lines()
        .filter_map(|line| Manual::from_str(line).ok())
        .collect::<Vec<Manual>>();

    let result = machines
        .iter()
        .filter_map(|m| m.solve_for_joltage())
        .sum::<usize>();

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
