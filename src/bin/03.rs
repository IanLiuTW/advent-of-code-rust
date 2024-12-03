advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = 0;

    for line in input.lines() {
        let mul_matches: Vec<_> = line.match_indices("mul(").collect();

        for (start_index, _) in mul_matches {
            if let Some(result) = parse_mul_expression(&line[start_index..]) {
                ans += result;
            }
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans = 0;

    let tokens = ["mul(", "do()", "don't()"];

    let mut should_count = true;

    for line in input.lines() {
        let mut matched_indices = vec![];

        for token in tokens {
            matched_indices.extend(line.match_indices(token));
        }
        matched_indices.sort();

        for (start_index, token) in matched_indices {
            match token {
                "mul(" => {
                    if should_count {
                        if let Some(result) = parse_mul_expression(&line[start_index..]) {
                            ans += result;
                        }
                    }
                }
                "do()" => should_count = true,
                "don't()" => should_count = false,
                _ => unreachable!(),
            }
        }
    }

    Some(ans)
}

fn parse_mul_expression(slice: &str) -> Option<u32> {
    let mul_content = slice.strip_prefix("mul(")?.split(')').next()?;
    let nums: Vec<&str> = mul_content.split(',').collect();

    if nums.len() != 2 {
        return None;
    }

    let first = nums[0].parse::<u32>().ok()?;
    let second = nums[1].parse::<u32>().ok()?;

    Some(first * second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
