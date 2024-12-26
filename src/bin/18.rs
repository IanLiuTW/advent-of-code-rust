use std::collections::HashMap;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    let commands = get_commands(input, false);
    solve(&commands)
}

pub fn part_two(input: &str) -> Option<u64> {
    let commands = get_commands(input, true);
    solve(&commands)
}

fn solve(commands: &[(&str, i64)]) -> Option<u64> {
    let mut hm: HashMap<i64, Vec<(i64, i64, bool)>> = HashMap::new();
    fn insert_hm(
        hm: &mut HashMap<i64, Vec<(i64, i64, bool)>>,
        k: i64,
        v1: i64,
        v2: i64,
        is_open: bool,
    ) {
        (*hm.entry(k).or_insert(vec![])).push((v1, v2, is_open));
    }

    let mut loc = [0, 0];
    for (i, (direction, len)) in commands.iter().enumerate() {
        match *direction {
            "U" | "D" => {
                let delta = if direction == &"U" { -1 } else { 1 };
                for _ in 0..len - 1 {
                    loc[0] += delta;
                    insert_hm(&mut hm, loc[0], loc[1], loc[1], true);
                }
                loc[0] += delta;
            }
            "R" => {
                insert_hm(
                    &mut hm,
                    loc[0],
                    loc[1],
                    loc[1] + len,
                    is_horizontal_line_open(&commands, i),
                );
                loc[1] += len;
            }
            "L" => {
                insert_hm(
                    &mut hm,
                    loc[0],
                    loc[1] - len,
                    loc[1],
                    is_horizontal_line_open(&commands, i),
                );
                loc[1] -= len;
            }
            _ => unreachable!(),
        }
    }

    let mut ans = 0;
    for v in hm.values_mut() {
        v.sort();
        while !v.is_empty() {
            let (r1, r2, is_open) = v.pop().unwrap();
            if !is_open {
                ans += r2 - r1 + 1;
                continue;
            }
            while !v.last().unwrap().2 {
                v.pop();
            }
            let (l1, _, _) = v.pop().unwrap();
            ans += r2 - l1 + 1;
        }
    }
    Some(ans as u64)
}
fn get_commands(input: &str, advanced_mode: bool) -> Vec<(&str, i64)> {
    let mut commands = vec![];
    input.lines().for_each(|line| {
        let mut line = line.split_ascii_whitespace();
        if !advanced_mode {
            commands.push((
                line.next().unwrap(),
                line.next().unwrap().parse::<i64>().unwrap(),
            ));
        } else {
            let line = line
                .last()
                .unwrap()
                .trim_matches(&['(', ')', '#'] as &[char]);
            commands.push((
                match &line[5..6] {
                    "0" => "R",
                    "1" => "D",
                    "2" => "L",
                    "3" => "U",
                    _ => unreachable!(),
                },
                i64::from_str_radix(&line[..5], 16).unwrap(),
            ));
        }
    });
    commands
}
fn is_horizontal_line_open(commands: &[(&str, i64)], i: usize) -> bool {
    fn get_i(i: usize, delta: isize, n: usize) -> usize {
        (n + i).wrapping_add_signed(delta) % n
    }
    let (d1, _) = commands[get_i(i, -1, commands.len())];
    let (d2, _) = commands[get_i(i, 1, commands.len())];
    d1 == d2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
