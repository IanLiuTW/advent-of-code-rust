use itertools::Itertools;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 0, 0)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 1, 1)
}

fn solve(input: &str, max_off: usize, max_off_cnt: u32) -> Option<u32> {
    let mut ans = 0;

    let arrays = get_arrays(input);
    for array in arrays {
        ans += find_mirror_val(array, max_off, max_off_cnt);
    }
    Some(ans)
}

fn get_arrays(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut arrays = vec![];

    let mut tmp = vec![];
    for line in input.lines() {
        if line.is_empty() {
            arrays.push(tmp.clone());
            tmp.clear();
        } else {
            tmp.push(line.chars().collect_vec());
        }
    }
    arrays.push(tmp.clone());
    arrays
}

fn find_mirror_val(mut array: Vec<Vec<char>>, max_off: usize, max_off_cnt: u32) -> u32 {
    if let Some(i) = find_mirror_idx(&array, max_off, max_off_cnt) {
        ((i + 1) * 100) as u32
    } else {
        array = transpose(array);
        let i = find_mirror_idx(&array, max_off, max_off_cnt).unwrap();
        (i + 1) as u32
    }
}

fn find_mirror_idx(array: &Vec<Vec<char>>, max_off: usize, max_off_cnt: u32) -> Option<usize> {
    (0..array.len() - 1).find(|&i| {
        let result = is_mirrored(array, i, max_off, max_off_cnt);
        if max_off == 0 {
            result
        } else {
            result && !is_mirrored(array, i, 0, 0)
        }
    })
}

fn is_mirrored(array: &Vec<Vec<char>>, i: usize, max_off: usize, max_off_cnt: u32) -> bool {
    let first = (0..=i).rev().map(|i| &array[i]);
    let second = (i + 1..array.len()).map(|i| &array[i]);
    let mut off_cnt = 0;
    for (l1, l2) in std::iter::zip(first, second) {
        let off = how_many_off(l1, l2);
        match off {
            0 => continue,
            _ if off <= max_off => {
                off_cnt += 1;
                if off_cnt > max_off_cnt {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

fn transpose<T>(array: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..array[0].len())
        .map(|i| array.iter().map(|row| row[i].clone()).collect_vec())
        .collect_vec()
}

fn how_many_off<T>(a1: &Vec<T>, a2: &Vec<T>) -> usize
where
    T: PartialEq,
{
    std::iter::zip(a1, a2).filter(|(c1, c2)| c1 != c2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }

    #[test]
    fn test_transpose() {
        let array = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(transpose(array), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    }
}
