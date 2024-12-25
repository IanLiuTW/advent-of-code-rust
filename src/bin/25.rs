use itertools::Itertools;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let (locks, keys) = parse_input(input);

    let ans = locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| Object::will_match(lock, key))
        .count();

    Some(ans as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> (Vec<Object>, Vec<Object>) {
    let mut locks = vec![];
    let mut keys = vec![];

    for section in input.split("\n\n") {
        let item = Object::new(section);

        match item {
            Object::Lock(_) => locks.push(item),
            Object::Key(_) => keys.push(item),
        };
    }

    (locks, keys)
}

#[derive(Debug)]
enum Object {
    Lock(Vec<usize>),
    Key(Vec<usize>),
}

impl Object {
    fn new(section: &str) -> Self {
        let section = section.lines().collect::<Vec<_>>();

        let combo: Vec<usize> = (0..section[0].len())
            .map(|j| {
                (0..section.len())
                    .filter(|&i| section[i].chars().nth(j) == Some('#'))
                    .count()
                    - 1
            })
            .collect();

        if section[0].chars().nth(0).unwrap() == '#' {
            Object::Lock(combo)
        } else {
            Object::Key(combo)
        }
    }

    fn will_match(lock: &Object, key: &Object) -> bool {
        if let (Object::Lock(lock_combo), Object::Key(key_combo)) = (lock, key) {
            lock_combo
                .iter()
                .zip(key_combo.iter())
                .all(|(l, k)| l + k <= 5)
        } else {
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
