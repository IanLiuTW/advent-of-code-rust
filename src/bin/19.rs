use cached::proc_macro::cached;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    let (patterns, designs) = parse_input(input);

    let mut ans = 0;
    for design in designs {
        if dfs(design, &patterns) {
            ans += 1;
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (patterns, designs) = parse_input(input);

    let mut ans = 0;
    for design in designs {
        ans += dfs2(design, &patterns);
    }

    Some(ans)
}

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut input = input.lines();

    let patterns = input
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let designs = input
        .skip(1)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    (patterns, designs)
}

#[cached(key = "String", convert = r#"{ design.clone() }"#)]
fn dfs(design: String, patterns: &[String]) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in patterns {
        if design.starts_with(pattern) && dfs(design[pattern.len()..].to_string(), patterns) {
            return true;
        }
    }

    false
}

#[cached(key = "String", convert = r#"{ design.clone() }"#)]
fn dfs2(design: String, patterns: &[String]) -> u64 {
    if design.is_empty() {
        return 1;
    }

    let mut ans = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            ans += dfs2(design[pattern.len()..].to_string(), patterns);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
