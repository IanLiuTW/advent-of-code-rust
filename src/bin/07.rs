advent_of_code::solution!(7);

use std::{cmp::Ordering, iter::zip};
use counter::Counter;

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}

fn solve(input: &str, advanced_rule: bool) -> Option<u32> {
    let mut cards = Vec::new();
    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        cards.push(Hand::new(
            hand.to_string(),
            bid.parse().unwrap(),
            advanced_rule,
        ));
    }
    cards.sort();

    Some(
        cards
            .iter()
            .enumerate()
            .map(|(i, c)| ((i + 1) as u32) * c.bid)
            .sum(),
    )
}

fn get_label_strength(a: &char, advanced_rule: bool) -> u32 {
    match a {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' if advanced_rule => 0,
        'J' => 11,
        'T' => 10,
        _ => a.to_digit(10).unwrap(),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Cate {
    Five = 50,
    Four = 40,
    Full = 35,
    Three = 30,
    TwoPair = 20,
    OnePair = 10,
    High = 0,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand: String,
    cate: Cate,
    bid: u32,
    advanced_rule: bool,
}

impl Hand {
    fn new(hand: String, bid: u32, advanced_rule: bool) -> Self {
        let cate = if !advanced_rule {
            Self::get_cate_normal(hand.clone())
        } else {
            Self::get_cate_advanced(hand.clone())
        };
        Hand { hand, cate, bid, advanced_rule }
    }

    fn get_cate_normal(hand: String) -> Cate {
        let cnter = hand.chars().collect::<Counter<_>>();
        Self::determine_cate_based_on_cnter(cnter)
    }

    fn get_cate_advanced(hand: String) -> Cate {
        let mut cnter = hand.chars().collect::<Counter<_>>();
        if let Some(wildcard) = cnter.remove(&'J') {
            if wildcard == 5 {
                cnter[&'J'] = 5;
            } else {
                let top = cnter.k_most_common_ordered(1)[0].0;
                cnter[&top] += wildcard;
            }
        }

        Self::determine_cate_based_on_cnter(cnter)
    }

    fn determine_cate_based_on_cnter(cnter: Counter<char>) -> Cate {
        let cnter: Counter<&usize> = cnter.values().collect::<Counter<_>>();
        match cnter {
            _ if cnter[&5] == 1 => Cate::Five,
            _ if cnter[&4] == 1 => Cate::Four,
            _ if cnter[&3] == 1 && cnter[&2] == 1 => Cate::Full,
            _ if cnter[&3] == 1 => Cate::Three,
            _ if cnter[&2] == 2 => Cate::TwoPair,
            _ if cnter[&2] == 1 => Cate::OnePair,
            _ => Cate::High,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cate != other.cate {
            return self.cate.cmp(&other.cate);
        }
        for (a, b) in zip(self.hand.chars(), other.hand.chars()) {
            if a != b {
                return get_label_strength(&a, self.advanced_rule)
                    .cmp(&get_label_strength(&b, self.advanced_rule));
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
