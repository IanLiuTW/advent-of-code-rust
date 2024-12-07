advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut ans = 0;

    let equations = parse_input(input);
    for eq in equations {
        if bt(eq.nums[0], &eq.answer, &eq.nums, 1) {
            ans += eq.answer;
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ans = 0;

    let equations = parse_input(input);
    for eq in equations {
        if bt2(eq.nums[0], &eq.answer, &eq.nums, 1) {
            ans += eq.answer;
        }
    }

    Some(ans)
}

#[derive(Debug)]
struct Equation {
    answer: u64,
    nums: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    let mut equations = vec![];
    for line in input.lines() {
        let (answer, nums) = line.split_once(':').unwrap();
        let eq = Equation {
            answer: answer.parse::<u64>().unwrap(),
            nums: nums
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect(),
        };
        equations.push(eq);
    }
    equations
}

fn bt(curr_val: u64, answer: &u64, nums: &Vec<u64>, i: usize) -> bool {
    if curr_val > *answer {
        return false;
    }
    if i == nums.len() {
        return curr_val == *answer;
    }

    let next_val = nums[i];

    bt(curr_val * next_val, answer, nums, i + 1) || bt(curr_val + next_val, answer, nums, i + 1)
}

fn bt2(curr_val: u64, answer: &u64, nums: &Vec<u64>, i: usize) -> bool {
    if curr_val > *answer {
        return false;
    }
    if i == nums.len() {
        return curr_val == *answer;
    }

    let next_val = nums[i];

    if let Some(concat_val) = concat_nums(curr_val, &next_val) {
        if bt2(concat_val, answer, nums, i + 1) {
            return true;
        }
    }

    bt2(curr_val * next_val, answer, nums, i + 1) || bt2(curr_val + next_val, answer, nums, i + 1)
}

fn concat_nums(num1: u64, num2: &u64) -> Option<u64> {
    let new_num = num1.to_string() + &num2.to_string();
    let new_num = new_num.parse::<u64>().ok()?;

    Some(new_num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
