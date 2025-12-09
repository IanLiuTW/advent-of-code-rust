use advent_of_code::union_find::UnionFind;
use itertools::Itertools;
use ordered_float::OrderedFloat;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn from_iter(mut iter: impl Iterator<Item = f64>) -> Option<Self> {
        Some(Self {
            x: iter.next()?,
            y: iter.next()?,
            z: iter.next()?,
        })
    }

    fn distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input);
    solve_fixed_edges(&points, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);
    solve_until_connected(&points)
}

fn get_sorted_edges(points: &[Point]) -> impl Iterator<Item = (f64, usize, usize)> + '_ {
    points
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((i, p1), (j, p2))| (p1.distance(p2), i, j))
        .sorted_unstable_by_key(|(d, _, _)| OrderedFloat(*d))
}

fn solve_fixed_edges(points: &[Point], edge_limit: usize) -> Option<u64> {
    let mut uf = UnionFind::new(points.len());

    get_sorted_edges(points)
        .take(edge_limit)
        .for_each(|(_, i, j)| {
            uf.union(i, j);
        });

    let result = uf
        .get_component_sizes()
        .sorted_unstable_by_key(|&c| std::cmp::Reverse(c))
        .take(3)
        .product::<usize>();

    Some(result as u64)
}

fn solve_until_connected(points: &[Point]) -> Option<u64> {
    let mut uf = UnionFind::new(points.len());

    let (_, i, j) = get_sorted_edges(points).find(|&(_, i, j)| {
        uf.union(i, j);
        uf.get_groups() == 1
    })?;

    Some((points[i].x * points[j].x) as u64)
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| {
            let nums = line.split(',').filter_map(|n| n.parse::<f64>().ok());
            Point::from_iter(nums)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let points = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = solve_fixed_edges(&points, 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
