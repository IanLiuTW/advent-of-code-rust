use std::collections::{HashSet, HashMap};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = 0;

    for line in input.lines() {
        let mut text = line.split(':').skip(1).next().unwrap().split('|');
        let winning: HashSet<_> = text.next().unwrap().split_ascii_whitespace().collect();
        let nums: HashSet<_> = text.next().unwrap().split_ascii_whitespace().collect();

        let matches = winning.intersection(&nums).count();
        if matches > 0 {
            ans += 2u32.pow((matches-1) as u32)
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut num_cards: HashMap<usize, u32> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let mut text = line.split(':').skip(1).next().unwrap().split('|');
        let winning: HashSet<_> = text.next().unwrap().split_ascii_whitespace().collect();
        let nums: HashSet<_> = text.next().unwrap().split_ascii_whitespace().collect();

        let matches = winning.intersection(&nums).count();
        *num_cards.entry(i).or_insert(0) += 1;
        for j in i+1..=i+matches {
            *num_cards.entry(j).or_insert(0) += num_cards[&i];
        }
    }
    Some(num_cards.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
