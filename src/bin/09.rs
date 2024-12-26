advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<i32> {
    solve(input, true)
}

fn solve(input: &str, begin: bool) -> Option<i32> {
    let mut ans = 0;
    for line in input.lines() {
        let v = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let diff = find_diff(&v, begin);
        ans += get_final_val(&v, begin, diff);
    }
    Some(ans)
}

fn find_diff(v: &Vec<i32>, begin: bool) -> i32 {
    if v.iter().all(|&x| x == 0) {
        return 0;
    }

    let mut new_v = Vec::new();
    for (a, b) in std::iter::zip(v.iter(), v.iter().skip(1)) {
        new_v.push(b - a);
    }

    let diff = find_diff(&new_v, begin);
    get_final_val(&new_v, begin, diff)
}

fn get_final_val(v: &Vec<i32>, begin: bool, diff: i32) -> i32 {
    match begin {
        true => v.first().unwrap() - diff,
        false => v.last().unwrap() + diff,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
