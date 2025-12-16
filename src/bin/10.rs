advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let mut nums = parse_input(input);

    Some(solve(&mut nums, 40) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut nums = parse_input(input);

    Some(solve(&mut nums, 50) as u64)
}

fn solve(current: &mut Vec<u8>, iterations: usize) -> usize {
    let mut next = Vec::with_capacity(current.len() * 2);

    for _ in 0..iterations {
        let mut i = 0;
        let len = current.len();

        while i < len {
            let val = current[i];
            let mut count = 1;

            while i + 1 < len && current[i + 1] == val {
                i += 1;
                count += 1;
            }

            next.push(count);
            next.push(val);

            i += 1;
        }

        std::mem::swap(current, &mut next);
        next.clear();
    }

    current.len()
}

fn parse_input(input: &str) -> Vec<u8> {
    input.trim().bytes().map(|b| b - b'0').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82350));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1166642));
    }
}
