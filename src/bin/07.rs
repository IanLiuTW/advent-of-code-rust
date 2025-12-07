use std::collections::HashMap;

advent_of_code::solution!(7);

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    start_col: usize,
    active_rows: Vec<Vec<usize>>,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut start_col = 0;
        let mut width = 0;
        let mut active_rows = Vec::new();

        for line in input.lines() {
            width = width.max(line.len());

            let mut row_splitters = Vec::new();

            for (c, char) in line.chars().enumerate() {
                match char {
                    'S' => {
                        start_col = c;
                    }
                    '^' => row_splitters.push(c),
                    _ => {}
                }
            }

            if !row_splitters.is_empty() {
                active_rows.push(row_splitters);
            }
        }

        Grid {
            width,
            start_col,
            active_rows,
        }
    }

    fn simulate(&self) -> (u64, u64) {
        let mut current_beams = HashMap::with_capacity(self.width);
        let mut next_beams = HashMap::with_capacity(self.width);

        current_beams.insert(self.start_col, 1);

        let mut split_count = 0;

        for splitters in &self.active_rows {
            next_beams.clone_from(&current_beams);

            for &splitter_col in splitters {
                if current_beams.contains_key(&splitter_col) {
                    split_count += 1;

                    next_beams.remove(&splitter_col);

                    if splitter_col > 0 {
                        *next_beams.entry(splitter_col - 1).or_insert(0) +=
                            current_beams.get(&splitter_col).unwrap_or(&0)
                    }
                    if splitter_col + 1 < self.width {
                        *next_beams.entry(splitter_col + 1).or_insert(0) +=
                            current_beams.get(&splitter_col).unwrap_or(&0)
                    }
                }
            }

            std::mem::swap(&mut current_beams, &mut next_beams);
        }

        let timeline_count = current_beams.values().sum();

        (split_count, timeline_count)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    let (split_count, _) = grid.simulate();

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    let (_, timeline_count) = grid.simulate();

    Some(timeline_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
