advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = 0;

    for (other, me) in parse_input(input) {
        let other = Shape::new(other);
        let me = Shape::new(me);

        ans += me.get_score(other);
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans = 0;

    for (other, outcome) in parse_input(input) {
        let other = Shape::new(other);
        let me = Shape::new_based_on_outcome(outcome, &other);

        ans += me.get_score(other);
    }

    Some(ans)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    fn new(input: &str) -> Self {
        match input {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissor,
            _ => unreachable!(),
        }
    }

    fn new_based_on_outcome(outcome: &str, other: &Shape) -> Self {
        match outcome {
            "X" => match other {
                Shape::Rock => Shape::Scissor,
                Shape::Paper => Shape::Rock,
                Shape::Scissor => Shape::Paper,
            },
            "Y" => *other,
            "Z" => match other {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissor,
                Shape::Scissor => Shape::Rock,
            },
            _ => unreachable!(),
        }
    }

    fn get_score(&self, other: Shape) -> u32 {
        let mut score = match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        };

        if *self == other {
            score += 3;
        } else if (*self == Shape::Rock && other == Shape::Scissor)
            || (*self == Shape::Paper && other == Shape::Rock)
            || (*self == Shape::Scissor && other == Shape::Paper)
        {
            score += 6;
        }

        score
    }
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let line = line.split_once(" ").unwrap();
            (line.0, line.1)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
