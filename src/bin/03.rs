use std::collections::HashSet;

advent_of_code::solution!(3);

type Coord = (i32, i32);

pub fn part_one(input: &str) -> Option<u64> {
    Some(visited_coords(input.trim().chars()).len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let count = input
        .trim()
        .chars()
        .enumerate()
        .fold(
            (HashSet::from([(0, 0)]), [(0, 0), (0, 0)]),
            |(mut visited, mut agents), (i, c)| {
                let active_agent = &mut agents[i % 2];

                match c {
                    '>' => active_agent.0 += 1,
                    '<' => active_agent.0 -= 1,
                    '^' => active_agent.1 -= 1,
                    'v' => active_agent.1 += 1,
                    _ => {}
                };

                visited.insert(*active_agent);

                (visited, agents)
            },
        )
        .0
        .len();

    Some(count as u64)
}

fn visited_coords(input: impl Iterator<Item = char>) -> HashSet<Coord> {
    std::iter::once((0, 0))
        .chain(input.scan((0, 0), |pos, c| {
            match c {
                '>' => pos.0 += 1,
                '<' => pos.0 -= 1,
                '^' => pos.1 -= 1,
                'v' => pos.1 += 1,
                _ => {}
            }
            Some(*pos)
        }))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
