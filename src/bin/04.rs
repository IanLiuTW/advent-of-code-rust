use md5::{Digest, Md5};
use rayon::prelude::*;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let input_bytes = input.trim().as_bytes();

    (1..u64::MAX).find(|&i| {
        let mut hasher = Md5::new();
        hasher.update(input_bytes);
        hasher.update(i.to_string().as_bytes());
        let result = hasher.finalize();

        result[0] == 0 && result[1] == 0 && (result[2] & 0xF0) == 0
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_bytes = input.trim().as_bytes();

    (1..u64::MAX).into_par_iter().find_first(|&i| {
        let mut hasher = Md5::new();
        hasher.update(input_bytes);
        hasher.update(i.to_string().as_bytes());
        let result = hasher.finalize();

        result[0] == 0 && result[1] == 0 && result[2] == 0
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(609043));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6742839));
    }
}
