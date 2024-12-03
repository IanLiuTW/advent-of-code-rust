advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans: u32 = 0;

    'outer: for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let asc = nums[0] < nums[1];
        for i in 1..nums.len() {
            let diff = nums[i] - nums[i - 1];
            if (asc && diff < 0) || (!asc && diff > 0) {
                continue 'outer;
            }
            if diff.abs() < 1 || diff.abs() > 3 {
                continue 'outer;
            }
        }
        ans += 1;
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    fn is_safe(prev_num: Option<i32>, curr_num: i32) -> bool {
        match prev_num {
            None => true,
            Some(prev_num) => (1..=3).contains(&(curr_num - prev_num)),
        }
    }

    fn is_a_safe_sequence(
        nums: &[i32],
        i: usize,
        prev_num: Option<i32>,
        has_tolerance: bool,
    ) -> bool {
        if i == nums.len() {
            return true;
        }

        let curr_num = nums[i];
        if !is_safe(prev_num, curr_num) {
            if has_tolerance {
                is_a_safe_sequence(nums, i + 1, prev_num, false)
            } else {
                false
            }
        } else if has_tolerance {
            is_a_safe_sequence(nums, i + 1, prev_num, false)
                || is_a_safe_sequence(nums, i + 1, Some(curr_num), true)
        } else {
            is_a_safe_sequence(nums, i + 1, Some(curr_num), false)
        }
    }

    let mut ans: u32 = 0;
    for line in input.lines() {
        let mut nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        if is_a_safe_sequence(&nums, 0, None, true) {
            ans += 1;
            continue;
        }
        nums.reverse();
        if is_a_safe_sequence(&nums, 0, None, true) {
            ans += 1;
        }
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
