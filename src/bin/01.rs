advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let result = input
        .trim()
        .chars()
        .filter_map(|c| match c {
            '(' => Some(1),
            ')' => Some(-1),
            _ => None,
        })
        .sum::<i32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    input
        .trim()
        .chars()
        .filter_map(|c| match c {
            '(' => Some(1),
            ')' => Some(-1),
            _ => None,
        })
        .scan(0, |floor, delta| {
            *floor += delta;
            Some(*floor)
        })
        .position(|floor| floor == -1)
        .map(|index| index + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(-3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
