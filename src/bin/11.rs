use cached::proc_macro::cached;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        get_initial_stones(input)
            .iter()
            .map(|stone| dfs(*stone, 25))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        get_initial_stones(input)
            .iter()
            .map(|stone| dfs(*stone, 75))
            .sum(),
    )
}

fn get_initial_stones(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|word| word.parse::<u64>().unwrap())
        .collect()
}

#[cached]
fn dfs(stone: u64, cnt: u32) -> u64 {
    if cnt == 0 {
        return 1;
    }

    if stone == 0 {
        return dfs(1, cnt - 1);
    }

    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let stone_str = stone_str.split_at(stone_str.len() / 2);
        return dfs(stone_str.0.parse::<u64>().unwrap(), cnt - 1)
            + dfs(stone_str.1.parse::<u64>().unwrap(), cnt - 1);
    }

    dfs(stone * 2024, cnt - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let stones = get_initial_stones(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(stones, vec![125, 17]);

        let stones = dfs(125, 1);
        assert_eq!(stones, 1);
        let stones = dfs(125, 2);
        assert_eq!(stones, 2);
        let stones = dfs(125, 3);
        assert_eq!(stones, 2);
        let stones = dfs(125, 4);
        assert_eq!(stones, 3);
        let stones = dfs(125, 5);
        assert_eq!(stones, 5);

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.is_some());
    }
}
