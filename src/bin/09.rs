use std::collections::HashMap;

advent_of_code::solution!(9);

struct DistanceMatrix {
    data: Vec<u32>,
    size: usize,
}

impl DistanceMatrix {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0; size * size],
            size,
        }
    }

    #[inline(always)]
    fn get(&self, from: usize, to: usize) -> u32 {
        self.data[from * self.size + to]
    }

    #[inline(always)]
    fn set(&mut self, from: usize, to: usize, dist: u32) {
        self.data[from * self.size + to] = dist;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, |a, b| a.min(b))
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, |a, b| a.max(b))
}

fn solve<F>(input: &str, compare: F) -> Option<u64>
where
    F: Fn(u32, u32) -> u32 + Copy,
{
    let (matrix, n) = parse_input(input);

    (0..n)
        .filter_map(|start_node| tsp_recursive(&matrix, start_node, 1 << start_node, n, compare))
        .reduce(compare)
        .map(|val| val as u64)
}

fn tsp_recursive<F>(
    matrix: &DistanceMatrix,
    current: usize,
    visited_mask: u32,
    n: usize,
    compare: F,
) -> Option<u32>
where
    F: Fn(u32, u32) -> u32 + Copy,
{
    if visited_mask.count_ones() as usize == n {
        return Some(0);
    }

    (0..n)
        .filter(|&next| (visited_mask & (1 << next)) == 0)
        .filter_map(|next| {
            let dist = matrix.get(current, next);
            if dist == 0 {
                return None;
            }

            tsp_recursive(matrix, next, visited_mask | (1 << next), n, compare)
                .map(|rest| dist + rest)
        })
        .reduce(compare)
}

fn parse_input(input: &str) -> (DistanceMatrix, usize) {
    let mut name_to_id: HashMap<&str, usize> = HashMap::new();
    let mut next_id = 0;

    let edges: Vec<_> = input
        .lines()
        .map(|line| {
            let (cities, dist_str) = line.split_once(" = ").unwrap();
            let (c1, c2) = cities.split_once(" to ").unwrap();
            let dist: u32 = dist_str.parse().unwrap();

            let id1 = *name_to_id.entry(c1).or_insert_with(|| {
                let i = next_id;
                next_id += 1;
                i
            });
            let id2 = *name_to_id.entry(c2).or_insert_with(|| {
                let i = next_id;
                next_id += 1;
                i
            });

            (id1, id2, dist)
        })
        .collect();

    let n = name_to_id.len();
    let mut matrix = DistanceMatrix::new(n);

    for (u, v, d) in edges {
        matrix.set(u, v, d);
        matrix.set(v, u, d);
    }

    (matrix, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(982));
    }
}
