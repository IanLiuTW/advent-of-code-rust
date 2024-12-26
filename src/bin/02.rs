use std::{collections::HashMap, str::Split};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = 0;
    for line in input.lines() {
        let (mut game_info, game_content) = extract_info(line);
        let game_id = game_info.nth(1).unwrap().parse::<u32>().unwrap();
        if !is_over_limit(game_content) {
            ans += game_id;
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans = 0;
    for line in input.lines() {
        let (_, game_content) = extract_info(line);
        let maxs = extract_max(game_content);
        ans += maxs.values().into_iter().fold(1, |a, b| a * b);
    }
    Some(ans)
}

fn extract_info(line: &str) -> (Split<'_, char>, Split<'_, char>) {
    let mut parts = line.split(':');
    (
        parts.next().unwrap().split(' '),
        parts.next().unwrap().split(';'),
    )
}

fn is_over_limit(game_content: Split<'_, char>) -> bool {
    let limits: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    for bunch in game_content {
        for cubes in bunch.split(',') {
            let mut parts: Split<'_, char> = cubes.trim().split(' ');
            let cnt: u32 = parts.next().unwrap().trim().parse::<u32>().unwrap();
            let color: &str = parts.next().unwrap().trim();
            if limits.get(color).unwrap() < &cnt {
                return true;
            }
        }
    }
    false
}

fn extract_max(game_content: Split<'_, char>) -> HashMap<&str, u32> {
    let mut cnter: HashMap<&str, u32> = HashMap::new();
    for bunch in game_content {
        for cubes in bunch.split(',') {
            let mut parts: Split<'_, char> = cubes.trim().split(' ');
            let cnt = parts.next().unwrap().trim().parse::<u32>().unwrap();
            let color = parts.next().unwrap().trim();

            let ent = cnter.entry(color).or_insert(cnt);
            *ent = (*ent).max(cnt);
        }
    }
    cnter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
