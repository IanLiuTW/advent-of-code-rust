use num::Integer;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let (direction, map) = get_directions_and_map(input);

    let mut ans = 0;
    let mut curr = String::from("AAA");
    for c in direction.chars().cycle() {
        curr = get_new_location(&curr, c, &map);
        ans += 1;
        if curr == "ZZZ" {
            break;
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (direction, map) = get_directions_and_map(input);

    let mut ans = 1;
    for location in get_all_srcs(input) {
        let mut steps = 0;
        let mut curr: String = location.to_string();
        for c in direction.chars().cycle() {
            curr = get_new_location(&curr, c, &map);
            steps += 1;
            if curr.ends_with('Z') {
                break;
            }
        }
        ans = ans.lcm(&steps);
    }
    Some(ans)
}

#[derive(Debug)]
struct LR {
    l: String,
    r: String,
}

fn parse_map(line: &str) -> (String, LR) {
    let (src, dst) = line.split_once('=').unwrap();
    let src = src.trim().to_string();
    let (l, r) = dst
        .trim()
        .trim_matches(|c| c == '(' || c == ')')
        .split_once(',')
        .unwrap();
    let l = l.trim().to_string();
    let r = r.trim().to_string();
    (src, LR { l, r })
}

fn get_new_location(curr: &String, c: char, map: &HashMap<String, LR>) -> String {
    match c {
        'L' => (*map[curr].l).to_string(),
        'R' => (*map[curr].r).to_string(),
        _ => unreachable!(),
    }
}

fn get_directions_and_map(input: &str) -> (&str, HashMap<String, LR>) {
    let mut input = input.lines();
    let directions = input.next().unwrap();
    input.next();

    let mut map = HashMap::new();
    for line in input {
        let (src, lr) = parse_map(line);
        map.insert(src, lr);
    }
    (directions, map)
}

fn get_all_srcs(input: &str) -> Vec<String> {
    let input = input.lines().skip(2);
    input
        .map(|line| {
            let (src, _) = parse_map(line);
            src
        })
        .filter(|x| x.ends_with('A'))
        .collect()
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
        assert_eq!(result, Some(6));
    }
}
