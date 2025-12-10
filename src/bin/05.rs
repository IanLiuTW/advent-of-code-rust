use itertools::Itertools;

advent_of_code::solution!(5);

type ValidationStrategy = fn(&str) -> bool;

fn has_three_vowels(s: &str) -> bool {
    s.chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .nth(2)
        .is_some()
}

fn has_double_letter(s: &str) -> bool {
    s.chars().tuple_windows().any(|(a, b)| a == b)
}

fn no_forbidden_substrings(s: &str) -> bool {
    s.chars()
        .tuple_windows()
        .all(|pair| !matches!(pair, ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')))
}

fn has_pair_of_two_letters(s: &str) -> bool {
    let bytes = s.as_bytes();

    s.as_bytes().windows(2).enumerate().any(|(i, pair)| {
        bytes[i + 2..]
            .windows(2)
            .any(|future_pair| future_pair == pair)
    })
}

fn has_sandwich_pattern(s: &str) -> bool {
    s.chars()
        .tuple_windows::<(_, _, _)>()
        .any(|(a, _, c)| a == c)
}

const STRATEGIES_PART_ONE: &[ValidationStrategy] =
    &[has_three_vowels, has_double_letter, no_forbidden_substrings];

const STRATEGIES_PART_TWO: &[ValidationStrategy] = &[has_pair_of_two_letters, has_sandwich_pattern];

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .filter(|&line| STRATEGIES_PART_ONE.iter().all(|rule| rule(line)))
        .count();

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .filter(|&line| STRATEGIES_PART_TWO.iter().all(|rule| rule(line)))
        .count();

    Some(result as u64)
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
        assert_eq!(result, Some(0));
    }
}
