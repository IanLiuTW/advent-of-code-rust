use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans: u32 = 0;
    let mut lines = input.lines();

    let mut prev_line: Option<&str> = None;
    let mut curr_line: Option<&str> = lines.next();
    let mut next_line: Option<&str> = lines.next();
    while curr_line.is_some() {
        let mut symbol_indices: HashSet<usize> = get_symbol_index(curr_line, is_symbol);
        symbol_indices.extend(get_symbol_index(prev_line, is_symbol));
        symbol_indices.extend(get_symbol_index(next_line, is_symbol));
        let affected_indices = get_affected_indices(symbol_indices);

        ans += get_affected_sum(curr_line.unwrap(), affected_indices);

        prev_line = curr_line;
        curr_line = next_line;
        next_line = lines.next();
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans: u32 = 0;
    let mut lines = input.lines();

    let mut prev_line: Option<&str> = None;
    let mut curr_line: Option<&str> = lines.next();
    let mut next_line: Option<&str> = lines.next();
    while curr_line.is_some() {
        let symbol_indices: HashSet<usize> = get_symbol_index(curr_line, is_star);
        for i in symbol_indices {
            ans += get_affected_product(i, [prev_line, curr_line, next_line]);
        }

        prev_line = curr_line;
        curr_line = next_line;
        next_line = lines.next();
    }
    Some(ans)
}

fn is_symbol(c: &char) -> bool {
    c != &'.' && !c.is_ascii_digit()
}

fn is_star(c: &char) -> bool {
    c == &'*'
}

fn get_symbol_index(line: Option<&str>, condition: fn(&char) -> bool) -> HashSet<usize> {
    match line {
        None => HashSet::new(),
        Some(line) => line
            .chars()
            .enumerate()
            .filter(|(_, c)| condition(c))
            .map(|(i, _)| i)
            .collect(),
    }
}

fn get_affected_indices(symbol_indices: HashSet<usize>) -> HashSet<usize> {
    let mut ans = HashSet::new();
    for i in symbol_indices {
        ans.insert(i);
        ans.insert(i - 1);
        ans.insert(i + 1);
    }
    ans
}

fn get_affected_sum(line: &str, affected_indices: HashSet<usize>) -> u32 {
    let mut ans = 0;

    let mut num = String::new();
    let mut affected = false;
    for (i, c) in line.chars().enumerate() {
        match c.is_ascii_digit() {
            true => {
                num.push(c);
                if affected_indices.contains(&i) {
                    affected = true
                }
            }
            false => {
                if affected {
                    ans += num.parse::<u32>().unwrap();
                }
                num.clear();
                affected = false;
            }
        }
    }
    if affected {
        ans += num.parse::<u32>().unwrap();
    }
    ans
}

fn get_affected_product(index: usize, lines: [Option<&str>; 3]) -> u32 {
    let affected_indices = HashSet::from([index - 1, index, index + 1]);
    let mut nums = HashSet::new();

    for line in lines {
        if line.is_none() {
            continue
        }
        let line = line.unwrap();

        let mut num = String::new();
        let mut affected = false;
        for (i, c) in line.chars().enumerate() {
            match c.is_ascii_digit() {
                true => {
                    num.push(c);
                    if affected_indices.contains(&i) {
                        affected = true
                    }
                }
                false => {
                    if affected {
                        nums.insert( num.parse::<u32>().unwrap() );
                    }
                    num.clear();
                    affected = false;
                }
            }
        }
        if affected {
            nums.insert( num.parse::<u32>().unwrap() );
        }
    }

    if nums.len() != 2 { 0 } else { nums.iter().fold(1, |accu, x| accu*x) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
