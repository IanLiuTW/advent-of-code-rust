advent_of_code::solution!(8);

#[derive(Clone, Copy)]
enum ParseState {
    Normal,
    Escape,
    Hex1,
    Hex2,
}

pub fn part_one(input: &str) -> Option<u64> {
    let diff = input
        .lines()
        .map(|line| {
            let (_, memory_len, code_len) =
                line.chars()
                    .fold((ParseState::Normal, 0, 0), |(state, mem, code), c| {
                        let next_code = code + 1;

                        match state {
                            ParseState::Normal => match c {
                                '"' => (ParseState::Normal, mem, next_code),
                                '\\' => (ParseState::Escape, mem, next_code),
                                _ => (ParseState::Normal, mem + 1, next_code),
                            },
                            ParseState::Escape => match c {
                                'x' => (ParseState::Hex1, mem, next_code),
                                _ => (ParseState::Normal, mem + 1, next_code),
                            },
                            ParseState::Hex1 => (ParseState::Hex2, mem, next_code),
                            ParseState::Hex2 => (ParseState::Normal, mem + 1, next_code),
                        }
                    });

            code_len - memory_len
        })
        .sum();

    Some(diff)
}

pub fn part_two(input: &str) -> Option<u64> {
    let diff = input
        .lines()
        .map(|line| {
            line.chars().fold(2, |cost, c| match c {
                '\\' | '"' => cost + 1,
                _ => cost,
            })
        })
        .sum();

    Some(diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
