use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, true))
}

fn solve(input: &str, is_digit_in_word: bool) -> u32 {
    let digits: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut ans = 0;
    for line in input.lines() {
        let mut first = ' ';
        let mut second = ' ';

        for (i, mut chr) in line.chars().enumerate() {
            if is_digit_in_word {
                for (&s, &d) in &digits {
                    if line[i..].starts_with(s) {
                        chr = d;
                        break;
                    }
                }
            }

            if chr.is_numeric() {
                if first == ' ' {
                    first = chr;
                }
                second = chr;
            }
        }

        let mut num: String = String::new();
        num.push(first);
        num.push(second);

        ans += num.parse::<u32>().unwrap();
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
