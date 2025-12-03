use std::cmp::Reverse;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let ans = parse_input(input)
        .map(|s| find_max_num_before_k_digits(s, 2))
        .sum();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ans = parse_input(input)
        .map(|s| find_max_num_before_k_digits(s, 12))
        .sum();
    Some(ans)
}

fn parse_input(input: &str) -> impl Iterator<Item = &str> + '_ {
    input.lines()
}

fn find_max_from_index(s: &str, start: usize, end: usize) -> (usize, char) {
    let (rel_i, val) = s[start..end]
        .char_indices()
        .max_by_key(|(i, d)| (*d, Reverse(*i)))
        .unwrap();
    (rel_i + start, val)
}

fn find_max_num_before_k_digits(s: &str, k: usize) -> u64 {
    let digits = (0..k)
        .rev()
        .scan(0, |start, k| {
            let (i, digit) = find_max_from_index(s, *start, s.len() - k);
            *start = i + 1;
            Some(digit)
        })
        .collect::<String>();
    digits.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }

    #[test]
    fn test_find_max_from_index() {
        assert_eq!(find_max_from_index("987654321111111", 0, 15), (0, '9'));
        assert_eq!(find_max_from_index("234234234234278", 0, 15), (14, '8'));
        assert_eq!(find_max_from_index("234834234234278", 0, 15), (3, '8'));
    }
}
