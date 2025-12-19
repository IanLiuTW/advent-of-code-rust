use serde_json::{Value, from_str};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i64> {
    let json = parse_input(input);

    Some(sum_recursive(&json, false))
}

pub fn part_two(input: &str) -> Option<i64> {
    let json = parse_input(input);

    Some(sum_recursive(&json, true))
}

fn sum_recursive(value: &Value, ignore_red: bool) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap_or(0),
        Value::Array(arr) => arr.iter().map(|v| sum_recursive(v, ignore_red)).sum(),
        Value::Object(map) => {
            if ignore_red && map.values().any(|v| v == "red") {
                return 0;
            }

            map.values().map(|v| sum_recursive(v, ignore_red)).sum()
        }
        _ => 0,
    }
}

fn parse_input(input: &str) -> Value {
    from_str(input.trim()).expect("Failed to parse JSON")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
