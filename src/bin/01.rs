advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);

    let mut ans = 0;
    for vals in input.iter() {
        ans = ans.max(vals.iter().sum());
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input);

    let mut sums = vec![];
    for vals in input.iter() {
        sums.push(vals.iter().sum::<u32>());
    }

    sums.sort();

    Some(sums.iter().rev().take(3).sum())
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45000));
    }
}
