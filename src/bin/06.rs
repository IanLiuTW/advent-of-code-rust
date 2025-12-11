use std::str::FromStr;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = LightGrid::new();

    grid.apply_commands(input, |cmd, cell| match cmd {
        CommandType::On => *cell = 1,
        CommandType::Off => *cell = 0,
        CommandType::Toggle => *cell ^= 1,
    });

    Some(grid.total_brightness())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = LightGrid::new();

    grid.apply_commands(input, |cmd, cell| match cmd {
        CommandType::On => *cell += 1,
        CommandType::Off => *cell = cell.saturating_sub(1),
        CommandType::Toggle => *cell += 2,
    });

    Some(grid.total_brightness())
}

const GRID_SIZE: usize = 1000;

struct LightGrid {
    data: Vec<u32>,
}

impl LightGrid {
    fn new() -> Self {
        Self {
            data: vec![0; GRID_SIZE * GRID_SIZE],
        }
    }

    fn apply_commands<F>(&mut self, input: &str, action: F)
    where
        F: Fn(CommandType, &mut u32),
    {
        for line in input.lines() {
            if let Ok(cmd) = Command::from_str(line) {
                for y in cmd.start.1..=cmd.end.1 {
                    let start_idx = y * GRID_SIZE + cmd.start.0;
                    let end_idx = y * GRID_SIZE + cmd.end.0;

                    if let Some(row_slice) = self.data.get_mut(start_idx..=end_idx) {
                        for cell in row_slice {
                            action(cmd.instruction, cell);
                        }
                    }
                }
            }
        }
    }

    fn total_brightness(&self) -> u64 {
        self.data.iter().map(|&x| x as u64).sum()
    }
}

type Pos = (usize, usize);

#[derive(Clone, Copy)]
enum CommandType {
    On,
    Off,
    Toggle,
}

struct Command {
    instruction: CommandType,
    start: Pos,
    end: Pos,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rest, instruction) = if let Some(r) = s.strip_prefix("turn on ") {
            (r, CommandType::On)
        } else if let Some(r) = s.strip_prefix("turn off ") {
            (r, CommandType::Off)
        } else if let Some(r) = s.strip_prefix("toggle ") {
            (r, CommandType::Toggle)
        } else {
            return Err(());
        };

        let (start_str, end_str) = rest.split_once(" through ").ok_or(())?;

        fn parse_pos(p: &str) -> Result<Pos, ()> {
            let (x, y) = p.split_once(',').ok_or(())?;
            Ok((x.parse().map_err(|_| ())?, y.parse().map_err(|_| ())?))
        }

        Ok(Command {
            instruction,
            start: parse_pos(start_str)?,
            end: parse_pos(end_str)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(998996));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1001996));
    }
}
