use std::collections::HashSet;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    let mut plane = Plane::new(2);
    plane.run(input);

    Some(plane.tail_visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut plane = Plane::new(10);
    plane.run(input);

    Some(plane.tail_visited.len() as u32)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(isize, isize);

struct Plane {
    rope: Vec<Pos>,
    tail_visited: HashSet<Pos>,
}

impl Plane {
    fn new(rope_len: usize) -> Self {
        Self {
            rope: vec![Pos(0, 0); rope_len],
            tail_visited: HashSet::from([Pos(0, 0)]),
        }
    }

    fn run(&mut self, input: &str) {
        input.lines().for_each(|line| {
            let (direction, cnt) = line.split_once(" ").unwrap();

            for _ in 0..cnt.parse().unwrap() {
                self.move_head(direction);
            }
        });
    }

    fn move_head(&mut self, direction: &str) {
        self.update_head(direction);
        self.update_rope();
        self.record_tail();
    }

    fn update_head(&mut self, direction: &str) {
        let delta = Plane::get_delta_from_direction(direction);

        self.rope[0] = Plane::get_new_pos(&self.rope[0], delta);
    }

    fn update_rope(&mut self) {
        for i in 0..self.rope.len() - 1 {
            let [front_pos, back_pos] = [self.rope[i], self.rope[i + 1]];

            if let Some(delta) = Plane::get_delta_if_not_touching(&front_pos, &back_pos) {
                self.rope[i + 1] = Plane::get_new_pos_for_knot(&back_pos, &delta);
            } else {
                break;
            }
        }
    }

    fn record_tail(&mut self) {
        self.tail_visited.insert(*self.rope.last().unwrap());
    }

    fn get_delta_from_direction(direction: &str) -> (isize, isize) {
        match direction {
            "U" => (-1, 0),
            "D" => (1, 0),
            "L" => (0, -1),
            "R" => (0, 1),
            _ => unreachable!(),
        }
    }

    fn get_new_pos(pos: &Pos, delta: (isize, isize)) -> Pos {
        Pos(pos.0 + delta.0, pos.1 + delta.1)
    }

    fn get_new_pos_for_knot(pos: &Pos, delta: &(isize, isize)) -> Pos {
        Pos(pos.0 + delta.0.signum(), pos.1 + delta.1.signum())
    }

    fn get_delta_if_not_touching(front_pos: &Pos, back_pos: &Pos) -> Option<(isize, isize)> {
        let delta = (front_pos.0 - back_pos.0, front_pos.1 - back_pos.1);

        Plane::is_delta_too_large(&delta).then_some(delta)
    }

    fn is_delta_too_large(delta: &(isize, isize)) -> bool {
        delta.0.abs() >= 2 || delta.1.abs() >= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(88));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }
}
