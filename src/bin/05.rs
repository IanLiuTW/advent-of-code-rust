use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (intervals, food) = parse_input(input);

    let result = food
        .filter(|&f| {
            let i = intervals.partition_point(|&[left, _]| left <= f);
            i != 0 && f <= intervals[i - 1][1]
        })
        .count();

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (intervals, _) = parse_input(input);

    let result = intervals
        .into_iter()
        .map(|[left, right]| right - left + 1)
        .sum();

    Some(result)
}

fn parse_input(input: &str) -> (Vec<[u64; 2]>, impl Iterator<Item = u64>) {
    let mut input = input.lines();

    let intervals = input
        .by_ref()
        .take_while(|&line| !line.is_empty())
        .map(|line| {
            let (left, right) = line.split_once('-').unwrap();
            [left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()]
        })
        .sorted_unstable_by_key(|item| item[0])
        .coalesce(|prev, curr| {
            if prev[1] >= curr[0] {
                Ok([prev[0], prev[1].max(curr[1])])
            } else {
                Err((prev, curr))
            }
        })
        .collect_vec();

    let food = input.map(|line| line.parse::<u64>().unwrap());

    (intervals, food)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
