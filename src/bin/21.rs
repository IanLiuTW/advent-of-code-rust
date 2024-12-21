use cached::proc_macro::cached;
use std::collections::HashMap;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 25)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(isize, isize);

struct NumPad {
    keypad: KeyPad,
}

impl NumPad {
    fn new() -> Self {
        let keypad_nums = vec![
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
            vec![' ', '0', 'A'],
        ];
        NumPad {
            keypad: KeyPad::new(keypad_nums),
        }
    }

    fn to_arrows_sequences(&self, sequence: String) -> Vec<String> {
        let mut cur = 'A';
        let mut sequences = vec!["".to_string()];

        for c in sequence.chars() {
            let paths = self.keypad.mappings.get(&cur).unwrap().get(&c).unwrap();

            let mut tmp_sequences = vec![];
            for sequence in sequences {
                for path in paths {
                    let mut result = sequence.clone();
                    result.push_str(path);
                    tmp_sequences.push(result);
                }
            }

            cur = c;
            sequences = tmp_sequences;
        }

        sequences
    }
}

struct ArrowPad {
    keypad: KeyPad,
}

impl ArrowPad {
    fn new() -> Self {
        let keypad_arrows = vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']];
        ArrowPad {
            keypad: KeyPad::new(keypad_arrows),
        }
    }
}

struct KeyPad {
    mappings: HashMap<char, HashMap<char, Vec<String>>>,
}

impl KeyPad {
    fn new(keypad: Vec<Vec<char>>) -> Self {
        let mut mappings = HashMap::new();
        let (m, n) = (keypad.len(), keypad[0].len());

        for (i, line) in keypad.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if let Some(paths) =
                    KeyPad::get_paths(&keypad, m as isize, n as isize, Pos(i as isize, j as isize))
                {
                    mappings.insert(*c, paths);
                }
            }
        }

        KeyPad { mappings }
    }

    const DIRECTIONS: [((isize, isize), char); 4] =
        [((1, 0), 'v'), ((-1, 0), '^'), ((0, 1), '>'), ((0, -1), '<')];

    fn get_paths(
        keypad: &Vec<Vec<char>>,
        m: isize,
        n: isize,
        start_pos: Pos,
    ) -> Option<HashMap<char, Vec<String>>> {
        let start_char = keypad[start_pos.0 as usize][start_pos.1 as usize];
        if start_char == ' ' {
            return None;
        }

        let mut paths: HashMap<char, Vec<String>> =
            HashMap::from([(start_char, vec!["A".to_string()])]);

        let mut queue = vec![(start_pos, String::new())];
        while !queue.is_empty() {
            let mut next_queue = Vec::new();
            let mut new_paths: HashMap<(char, Pos), Vec<String>> = HashMap::new();

            for (current_pos, current_path) in queue {
                for (delta, direction_symbol) in KeyPad::DIRECTIONS {
                    let next_pos = Pos(current_pos.0 + delta.0, current_pos.1 + delta.1);

                    if !KeyPad::is_in_bound(m, n, &next_pos) {
                        continue;
                    }

                    let next_char = keypad[next_pos.0 as usize][next_pos.1 as usize];
                    if next_char == ' ' || paths.contains_key(&next_char) {
                        continue;
                    }

                    let mut new_path = current_path.clone();

                    if !new_path.is_empty()
                        && direction_symbol != new_path.chars().last().unwrap()
                        && direction_symbol == new_path.chars().next().unwrap()
                    {
                        continue;
                    }

                    new_path.push(direction_symbol);

                    new_paths
                        .entry((next_char, next_pos))
                        .or_default()
                        .push(new_path);
                }
            }

            for ((next_char, next_pos), paths_to_char) in new_paths {
                for path in paths_to_char {
                    next_queue.push((next_pos, path.clone()));

                    let mut final_path = path.clone();
                    final_path.push('A');
                    paths.entry(next_char).or_default().push(final_path);
                }
            }

            queue = next_queue;
        }

        Some(paths)
    }

    fn is_in_bound(m: isize, n: isize, pos: &Pos) -> bool {
        (0..m).contains(&pos.0) && (0..n).contains(&pos.1)
    }
}

pub fn solve(input: &str, level: u8) -> Option<u64> {
    let numpad = NumPad::new();
    let arrowpad = ArrowPad::new();

    let mut ans = 0;

    for line in input.lines() {
        let mut min_ = u64::MAX;

        let sequences = numpad.to_arrows_sequences(line.to_string());
        for sequence in sequences {
            let mut sum_ = dp(&arrowpad, 'A', sequence.chars().next().unwrap(), level);
            for (start, end) in sequence.chars().zip(sequence.chars().skip(1)) {
                sum_ += dp(&arrowpad, start, end, level);
            }

            min_ = min_.min(sum_);
        }

        let num = line[0..3].parse::<u64>().unwrap();
        ans += min_ * num;
    }

    Some(ans)
}

#[cached(key = "(char, char, u8)", convert = r#"{ (start, end, level) }"#)]
fn dp(arrowpad: &ArrowPad, start: char, end: char, level: u8) -> u64 {
    let paths = arrowpad
        .keypad
        .mappings
        .get(&start)
        .unwrap()
        .get(&end)
        .unwrap();

    if level == 1 {
        return paths.iter().next().unwrap().len() as u64;
    }

    let mut ans = u64::MAX;

    let mut cur = 'A';
    for path in paths {
        let mut sum_ = 0;

        for c in path.chars() {
            sum_ += dp(arrowpad, cur, c, level - 1);
            cur = c;
        }

        ans = ans.min(sum_);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
