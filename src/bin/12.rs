advent_of_code::solution!(12);

use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u64> {
    let (shapes, queries) = parse_input(input);

    let mut success_count = 0;

    for (width, height, mut counts) in queries {
        let region_area = width * height;

        let mut presents_area = 0;
        for (id, &count) in counts.iter().enumerate() {
            if id < shapes.len() {
                presents_area += shapes[id].area * count as isize;
            }
        }

        if presents_area > region_area {
            continue;
        }

        let slack = (region_area - presents_area) as usize;

        let mut solver = Solver::new(&shapes, height, width);
        if solver.solve(&mut counts, slack) {
            success_count += 1;
        }
    }

    Some(success_count)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

type Point = (isize, isize);

#[derive(Clone)]
struct ShapeInfo {
    variants: Vec<ShapeVariant>,
    area: isize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ShapeVariant {
    offsets: Vec<Point>,
}

struct Solver<'a> {
    rows: isize,
    cols: isize,
    grid: Vec<bool>,
    shapes: &'a [ShapeInfo],
}

impl<'a> Solver<'a> {
    fn new(shapes: &'a [ShapeInfo], rows: isize, cols: isize) -> Self {
        Solver {
            rows,
            cols,
            grid: vec![false; (rows * cols) as usize],
            shapes,
        }
    }

    fn solve(&mut self, counts: &mut [usize], skips_left: usize) -> bool {
        let mut empty_idx = None;

        for i in 0..self.grid.len() {
            if !self.grid[i] {
                empty_idx = Some(i);
                break;
            }
        }

        let idx = match empty_idx {
            Some(i) => i,
            None => return true,
        };

        if counts.iter().all(|&c| c == 0) {
            return true;
        }

        let r = (idx as isize) / self.cols;
        let c = (idx as isize) % self.cols;

        for shape_id in 0..counts.len() {
            if counts[shape_id] > 0 {
                for variant in &self.shapes[shape_id].variants {
                    if self.can_place(r, c, variant) {
                        self.place(r, c, variant, true);
                        counts[shape_id] -= 1;

                        if self.solve(counts, skips_left) {
                            return true;
                        }

                        counts[shape_id] += 1;
                        self.place(r, c, variant, false);
                    }
                }
            }
        }

        if skips_left > 0 {
            self.grid[idx] = true;
            if self.solve(counts, skips_left - 1) {
                return true;
            }
            self.grid[idx] = false;
        }

        false
    }

    #[inline(always)]
    fn can_place(&self, r: isize, c: isize, variant: &ShapeVariant) -> bool {
        for &(dr, dc) in &variant.offsets {
            let nr = r + dr;
            let nc = c + dc;

            if nr < 0 || nr >= self.rows || nc < 0 || nc >= self.cols {
                return false;
            }
            if self.grid[(nr * self.cols + nc) as usize] {
                return false;
            }
        }
        true
    }

    #[inline(always)]
    fn place(&mut self, r: isize, c: isize, variant: &ShapeVariant, state: bool) {
        for &(dr, dc) in &variant.offsets {
            let nr = r + dr;
            let nc = c + dc;

            self.grid[(nr * self.cols + nc) as usize] = state;
        }
    }
}

fn parse_input(input: &str) -> (Vec<ShapeInfo>, Vec<(isize, isize, Vec<usize>)>) {
    let mut shapes_map: std::collections::BTreeMap<usize, Vec<String>> =
        std::collections::BTreeMap::new();
    let mut queries = Vec::new();
    let mut current_shape_idx = None;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.contains("x") && line.contains(":") {
            let parts: Vec<&str> = line.split(':').collect();
            let dims: Vec<&str> = parts[0].split('x').collect();

            let w = dims[0].parse().unwrap();
            let h = dims[1].parse().unwrap();

            let counts: Vec<usize> = parts[1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            queries.push((w, h, counts));
        } else if let Some(idx_str) = line.strip_suffix(":") {
            if let Ok(idx) = idx_str.parse::<usize>() {
                current_shape_idx = Some(idx);
                shapes_map.entry(idx).or_default();
            }
        } else if let Some(idx) = current_shape_idx {
            shapes_map.entry(idx).or_default().push(line.to_string());
        }
    }

    let shapes: Vec<ShapeInfo> = shapes_map
        .values()
        .map(|lines| {
            let coords = parse_shape_grid(lines);
            let area = coords.len() as isize;
            let variants = generate_variants(&coords);

            ShapeInfo { variants, area }
        })
        .collect();

    (shapes, queries)
}

fn parse_shape_grid(lines: &[String]) -> Vec<Point> {
    let mut coords = Vec::new();

    for (r, line) in lines.iter().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char == '#' {
                coords.push((r as isize, c as isize));
            }
        }
    }

    coords
}

fn generate_variants(base_coords: &[Point]) -> Vec<ShapeVariant> {
    let mut unique = HashSet::new();
    let mut current = base_coords.to_vec();

    for _ in 0..4 {
        let rotated: Vec<Point> = current.iter().map(|(r, c)| (*c, -*r)).collect();
        unique.insert(normalize(&rotated));

        let flipped: Vec<Point> = rotated.iter().map(|(r, c)| (*r, -*c)).collect();
        unique.insert(normalize(&flipped));

        current = rotated;
    }

    unique.into_iter().collect()
}

fn normalize(coords: &[Point]) -> ShapeVariant {
    let min_r = coords.iter().map(|p| p.0).min().unwrap();
    let min_c = coords.iter().map(|p| p.1).min().unwrap();

    let mut shifted: Vec<Point> = coords.iter().map(|(r, c)| (r - min_r, c - min_c)).collect();
    shifted.sort();

    let anchor_r = shifted[0].0;
    let anchor_c = shifted[0].1;

    let final_coords = shifted
        .iter()
        .map(|(r, c)| (r - anchor_r, c - anchor_c))
        .collect();

    ShapeVariant {
        offsets: final_coords,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
