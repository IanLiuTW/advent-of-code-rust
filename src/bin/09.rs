use rayon::prelude::*;

advent_of_code::solution!(9);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn area_with(&self, other: Point) -> u64 {
        let width = self.x.abs_diff(other.x) + 1;
        let height = self.y.abs_diff(other.y) + 1;

        width * height
    }
}

struct Polygon {
    vertical_edges: Vec<(u64, u64, u64)>,
    horizontal_edges: Vec<(u64, u64, u64)>,
}

impl Polygon {
    fn new(points: Vec<Point>) -> Self {
        let n = points.len();

        let mut vertical_edges = Vec::new();
        let mut horizontal_edges = Vec::new();

        for i in 0..n {
            let p1 = points[i];
            let p2 = points[(i + 1) % n];

            if p1.x == p2.x {
                let y_min = p1.y.min(p2.y);
                let y_max = p1.y.max(p2.y);

                vertical_edges.push((p1.x, y_min, y_max));
            } else if p1.y == p2.y {
                let x_min = p1.x.min(p2.x);
                let x_max = p1.x.max(p2.x);

                horizontal_edges.push((p1.y, x_min, x_max));
            }
        }

        Self {
            vertical_edges,
            horizontal_edges,
        }
    }

    fn is_valid_rect(&self, p1: Point, p2: Point) -> bool {
        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_y = p1.y.max(p2.y);

        let center_x = (min_x as f64 + max_x as f64) / 2.0;
        let center_y = (min_y as f64 + max_y as f64) / 2.0;

        self.contains_point(center_x, center_y) // 1. TOPOLOGY CHECK: Raycast from Center
            && !self.has_internal_edge(min_x, max_x, min_y, max_y) // 2. INTEGRITY CHECK: Edge Intersection
    }

    fn contains_point(&self, x: f64, y: f64) -> bool {
        let mut intersections = 0;
        for &(edge_x, y_min, y_max) in &self.vertical_edges {
            if (edge_x as f64) > x && (y >= y_min as f64) && (y <= y_max as f64) {
                intersections += 1;
            }
        }

        intersections % 2 != 0
    }

    fn has_internal_edge(&self, min_x: u64, max_x: u64, min_y: u64, max_y: u64) -> bool {
        for &(vx, vy_min, vy_max) in &self.vertical_edges {
            if vx > min_x && vx < max_x {
                let overlap_start = vy_min.max(min_y);
                let overlap_end = vy_max.min(max_y);

                if overlap_start < overlap_end {
                    return true;
                }
            }
        }

        for &(hy, hx_min, hx_max) in &self.horizontal_edges {
            if hy > min_y && hy < max_y {
                let overlap_start = hx_min.max(min_x);
                let overlap_end = hx_max.min(max_x);

                if overlap_start < overlap_end {
                    return true;
                }
            }
        }

        false
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points: Vec<Point> = parse_input(input).collect();

    points
        .par_iter()
        .enumerate()
        .map(|(i, &p1)| {
            points[i + 1..]
                .iter()
                .map(move |&p2| p1.area_with(p2))
                .max()
                .unwrap_or(0)
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<Point> = parse_input(input).collect();
    let polygon = Polygon::new(points.clone());

    points
        .par_iter()
        .enumerate()
        .map(|(i, &p1)| {
            points[i + 1..]
                .iter()
                .filter_map(|&p2| (polygon.is_valid_rect(p1, p2)).then_some(p1.area_with(p2)))
                .max()
                .unwrap_or(0)
        })
        .max()
}

fn parse_input(input: &str) -> impl Iterator<Item = Point> + '_ {
    input.lines().filter_map(|line| {
        let (x_str, y_str) = line.split_once(',')?;
        Some(Point {
            x: x_str.parse::<u64>().ok()?,
            y: y_str.parse::<u64>().ok()?,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
