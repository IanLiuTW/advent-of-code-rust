use advent_of_code::solution;

const CIRCUMFERENCE: i64 = 100;
const START_POS: i64 = 50;

#[derive(Debug, Clone, Copy)]
enum Command {
    Right(i64),
    Left(i64),
}

impl Command {
    fn apply(&self, current: i64) -> i64 {
        match self {
            Command::Right(v) => current + v,
            Command::Left(v) => current - v,
        }
    }
}

solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let result = parse_input(input)
        .scan(START_POS, |state, cmd| {
            *state = cmd.apply(*state);
            *state = state.rem_euclid(CIRCUMFERENCE);

            Some(*state)
        })
        .filter(|&pos| pos == 0)
        .count();

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = parse_input(input)
        .scan(START_POS, |state, cmd| {
            let next_state = cmd.apply(*state);

            let crossings = if next_state > *state {
                next_state.div_euclid(CIRCUMFERENCE) - state.div_euclid(CIRCUMFERENCE)
            } else {
                (*state - 1).div_euclid(CIRCUMFERENCE) - (next_state - 1).div_euclid(CIRCUMFERENCE)
            };

            *state = next_state.rem_euclid(CIRCUMFERENCE);

            Some(crossings as u64)
        })
        .sum();

    Some(result)
}

fn parse_input(input: &str) -> impl Iterator<Item = Command> + '_ {
    input.lines().filter_map(|line| {
        let (dir_str, count_str) = line.split_at_checked(1)?;
        let count = count_str.parse::<i64>().ok()?;

        match dir_str {
            "R" => Some(Command::Right(count)),
            "L" => Some(Command::Left(count)),
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
