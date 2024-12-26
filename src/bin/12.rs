advent_of_code::solution!(12);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    // solve(input, false, brute_force)
    solve(input, false, dfa)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true, dfa)
}

fn solve(
    input: &str,
    advanced_mode: bool,
    algo: impl Fn(&Vec<char>, &Vec<usize>) -> u64,
) -> Option<u64> {
    let mut ans = 0;
    for line in input.lines() {
        let (record, cnt) = parse_line(line, advanced_mode);
        ans += algo(&record, &cnt);
    }
    Some(ans)
}

fn parse_line(line: &str, advanced_mode: bool) -> (Vec<char>, Vec<usize>) {
    let mut line = line.split_ascii_whitespace();
    let record = line.next().unwrap().chars().collect();
    let cnt = line
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    if !advanced_mode {
        (record, cnt)
    } else {
        let mut new_record = record.clone();
        let mut new_cnt = cnt.clone();
        for _ in 0..4 {
            new_record.push('?');
            new_record.extend(record.iter());
            new_cnt.extend(cnt.iter());
        }

        (new_record, new_cnt)
    }
}

fn brute_force(record: &Vec<char>, cnt: &Vec<usize>) -> u64 {
    fn bt(ans: &mut u64, re: &Regex, combo: &mut Vec<char>, record: &Vec<char>, i: usize) {
        if i == record.len() {
            if re.is_match(&combo.iter().collect::<String>()) {
                *ans += 1;
            }
            return;
        }
        for chr in ['.', '#'] {
            if [chr, '?'].contains(&record[i]) {
                combo.push(chr);
                bt(ans, re, combo, record, i + 1);
                combo.pop();
            }
        }
    }

    let mut ans = 0;
    let re = Regex::new(&get_regex(cnt)).unwrap();
    bt(&mut ans, &re, &mut vec![], record, 0);

    ans
}

fn get_regex(cnt: &Vec<usize>) -> String {
    let mut re: Vec<String> = vec![];
    for &num in cnt {
        re.push("#".repeat(num));
    }
    format!("^[.]*{}[.]*$", re.join("[.]+"))
}

fn dfa(record: &Vec<char>, cnt: &Vec<usize>) -> u64 {
    let states = get_dfa_states(cnt);

    let n = states.len();
    let mut dfa = vec![0; n];
    dfa[0] = 1;

    for chr in record {
        let mut new_dfa = vec![0; n];
        for (i, state) in states.iter().enumerate() {
            if ['.', '?'].contains(chr) {
                if *state == '#' && states[i + 1] != '#' {
                    new_dfa[i + 1] += dfa[i];
                } else if *state == '*' {
                    new_dfa[i] += dfa[i];
                }
            }
            if ['#', '?'].contains(chr)
                && ((*state == '#' && states[i + 1] == '#') || (*state == '*' && i + 1 < n))
            {
                new_dfa[i + 1] += dfa[i];
            }
        }
        dfa = new_dfa;
    }
    dfa[n - 2..].iter().sum()
}

fn get_dfa_states(cnt: &Vec<usize>) -> Vec<char> {
    let mut dfa: Vec<char> = vec!['*'];
    for &num in cnt {
        dfa.resize(dfa.len() + num, '#');
        dfa.push('*');
    }
    dfa
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }

    #[test]
    fn test_dfa() {
        let states = get_dfa_states(&vec![1, 1, 3]);
        assert_eq!(states, "*#*#*###*".chars().collect::<Vec<char>>());

        let (record, cnt) = parse_line("?###???????? 3,2,1", false);
        let ans = dfa(&record, &cnt);
        assert_eq!(ans, 10);
    }

    #[test]
    fn test_parse_line() {
        let (record, cnt) = parse_line(".# 1", true);
        assert_eq!(record, ".#?.#?.#?.#?.#".chars().collect::<Vec<_>>());
        assert_eq!(cnt, vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_advanced_mode() {
        let (record, cnt) = parse_line(".??..??...?##. 1,1,3", true);
        let ans = dfa(&record, &cnt);
        assert_eq!(ans, 16384);
    }
}
