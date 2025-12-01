use std::collections::{HashMap, HashSet};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::new(input);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct Map {
    obstacles: HashMap<usize, HashSet<usize>>,
    x_top_obs: HashMap<usize, usize>,
    end_state: bool,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut obstacles: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut x_top_obs = HashMap::new();

        for line in input.lines() {
            let mut points = line.split(" -> ").map(|point| {
                let xy = point.split_once(",").unwrap();
                (
                    xy.0.parse::<usize>().unwrap(),
                    xy.1.parse::<usize>().unwrap(),
                )
            });

            let (mut x, mut y) = points.next().unwrap();
            for (nx, ny) in points {
                if x == nx {
                    (y.min(ny)..=y.max(ny)).for_each(|y| {
                        obstacles.entry(x).or_default().insert(y);
                    });
                } else {
                    (x.min(nx)..=x.max(nx)).for_each(|x| {
                        obstacles.entry(x).or_default().insert(y);
                    });
                }

                (x, y) = (nx, ny);
            }
        }

        for (x, ys) in obstacles.iter() {
            x_top_obs.insert(*x, *ys.iter().min().unwrap());
        }

        Map {
            obstacles,
            x_top_obs,
            end_state: false,
        }
    }

    fn drop_sand(&mut self, x: usize, y: usize) {
        if let Some(top_y) = self.x_top_obs.get(&x) {
            let y = *top_y - 1;
            todo!();
        } else {
            self.end_state = true;
        }
    }

    fn drop_sand_from_top() {
        todo!()
    }

    fn sand_fall(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if let Some(ys) = self.obstacles.get(&x) {
            todo!()
        } else {
            None
        }
    }

    fn has_obstacle(&self, x: usize, y: usize) -> bool {
        if let Some(ys) = self.obstacles.get(&x) {
            ys.contains(&y)
        } else {
            false
        }
    }

    fn update_x_top_obs(&mut self, x: usize, new_y: usize) {
        if let Some(val) = self.x_top_obs.get_mut(&x) {
            *val = new_y.min(*val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_map() {
        let mut map = Map {
            obstacles: HashMap::from([(3, HashSet::from([3]))]),
            x_top_obs: HashMap::from([(3, 3)]),
            end_state: false,
        };

        assert!(map.has_obstacle(3, 3));
        assert!(!map.has_obstacle(3, 4));

        map.update_x_top_obs(3, 4);
        assert_eq!(*map.x_top_obs.get(&3).unwrap(), 3);
        map.update_x_top_obs(3, 1);
        assert_eq!(*map.x_top_obs.get(&3).unwrap(), 1);
    }
}
