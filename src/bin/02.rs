use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, |repeats| repeats == 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, |repeats| repeats >= 2)
}

fn solve<F>(input: &str, valid_repetition: F) -> Option<u64>
where
    F: Fn(u32) -> bool,
{
    let mut invalid_ids = HashSet::new();

    for (start, end) in parse_input(input) {
        let start_digits = count_digits(start);
        let end_digits = count_digits(end);

        for total_len in start_digits..=end_digits {
            let min_in_len = if total_len == start_digits {
                start
            } else {
                10u64.pow(total_len - 1)
            };
            let max_in_len = if total_len == end_digits {
                end
            } else {
                10u64.pow(total_len) - 1
            };

            for chunk_len in 1..=(total_len / 2) {
                if !total_len.is_multiple_of(chunk_len) {
                    continue;
                }

                let repeats = total_len / chunk_len;
                if !valid_repetition(repeats) {
                    continue;
                }

                let shift = total_len - chunk_len;
                let min_base = min_in_len / 10u64.pow(shift);
                let max_base = max_in_len / 10u64.pow(shift);

                for base in min_base..=max_base {
                    let candidate = generate_repeated_number(base, repeats, chunk_len);

                    if min_in_len <= candidate && candidate <= max_in_len {
                        invalid_ids.insert(candidate);
                    }
                }
            }
        }
    }

    Some(invalid_ids.iter().sum())
}

fn parse_input(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    input.split(',').filter_map(|s| {
        let (a, b) = s.trim().split_once('-')?;
        Some((a.parse().ok()?, b.parse().ok()?))
    })
}

fn generate_repeated_number(base: u64, repeats: u32, width: u32) -> u64 {
    let mut result = 0;
    let multiplier = 10u64.pow(width);
    for _ in 0..repeats {
        result = result * multiplier + base;
    }
    result
}

fn count_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
