use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_nums = vec![];
    let mut right_nums = vec![];
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        left_nums.push(words[0].parse::<i32>().unwrap());
        right_nums.push(words[1].parse::<i32>().unwrap());
    }
    left_nums.sort();
    right_nums.sort();

    let mut ans: u32 = 0;
    for (left, right) in left_nums.iter().zip(right_nums.iter()) {
        ans += (right - left).abs() as u32;
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_nums = vec![];
    let mut right_nums = vec![];
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        left_nums.push(words[0].parse::<i32>().unwrap());
        right_nums.push(words[1].parse::<i32>().unwrap());
    }

    let mut cnter: HashMap<i32, u32> = HashMap::new();
    for num in right_nums {
        *cnter.entry(num).or_insert(0) += 1;
    }

    let mut ans: u32 = 0;
    for num in left_nums {
        ans += num as u32 * *cnter.entry(num).or_insert(0);
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
