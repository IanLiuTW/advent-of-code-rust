use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::new(input);
    map.beam_travel(0, 0, Beam(Direction::Right));
    Some(map.count_energized())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans = 0;
    let mut map = Map::new(input);

    for (iter, beam) in [
        ((0..map.m).cartesian_product(0..1), Beam(Direction::Right)),
        (
            (0..map.m).cartesian_product(map.n - 1..map.n),
            Beam(Direction::Left),
        ),
        ((0..1).cartesian_product(0..map.n), Beam(Direction::Down)),
        (
            (map.m - 1..map.m).cartesian_product(0..map.n),
            Beam(Direction::Up),
        ),
    ] {
        for (i, j) in iter {
            map.beam_travel(i, j, beam);
            ans = ans.max(map.count_energized());
            map.reset();
        }
    }
    Some(ans)
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Tile>>,
    m: usize,
    n: usize,
}
impl Map {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().map(|c| Tile::new(&c)).collect_vec())
            .collect_vec();
        let m = map.len();
        let n = map[0].len();
        Self { map, m, n }
    }
    fn beam_travel(&mut self, i: usize, j: usize, beam: Beam) {
        let next_beams = self.map[i][j].react_to_beam(beam);
        // map.show_heatmap();
        for beam in next_beams {
            let Beam(direction) = beam;
            if let Some((ni, nj)) = self.get_next_pos(i, j, direction) {
                self.beam_travel(ni, nj, beam);
            }
        }
    }
    fn get_next_pos(
        &self,
        mut i: usize,
        mut j: usize,
        direction: Direction,
    ) -> Option<(usize, usize)> {
        let (di, dj) = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        i = i.checked_add_signed(di)?;
        j = j.checked_add_signed(dj)?;
        if i >= self.m || j >= self.n {
            return None;
        }
        Some((i, j))
    }
    fn count_energized(&self) -> u32 {
        self.map
            .iter()
            .map(|row| row.iter().filter(|tile| tile.is_energized()).count())
            .sum::<usize>() as u32
    }
    fn reset(&mut self) {
        for row in self.map.iter_mut() {
            for tile in row.iter_mut() {
                tile.remove_all_beams();
            }
        }
    }
    #[allow(dead_code)]
    fn show_heatmap(&self) {
        let display = self
            .map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| if tile.is_energized() { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect_vec();
        for line in display {
            println!("{}", line);
        }
    }
}

#[derive(Debug)]
enum TileType {
    Empty,
    MirrorLeft,
    MirrorRight,
    SplitterVertical,
    SplitterHorizontal,
}
#[derive(Debug)]
struct Tile(TileType, HashSet<Beam>);
impl Tile {
    fn new(c: &char) -> Self {
        let type_ = match c {
            '.' => TileType::Empty,
            '/' => TileType::MirrorLeft,
            '\\' => TileType::MirrorRight,
            '|' => TileType::SplitterVertical,
            '-' => TileType::SplitterHorizontal,
            _ => unreachable!(),
        };
        Self(type_, HashSet::new())
    }
    fn react_to_beam(&mut self, beam: Beam) -> Vec<Beam> {
        let mut ans = vec![];
        if self.has_beam(&beam) {
            return ans;
        }
        self.add_beam(beam);

        let Beam(direction) = beam;
        match self.0 {
            TileType::Empty => {
                ans.push(Beam(direction));
            }
            TileType::MirrorLeft => match direction {
                Direction::Up => ans.push(Beam(Direction::Right)),
                Direction::Left => ans.push(Beam(Direction::Down)),
                Direction::Down => ans.push(Beam(Direction::Left)),
                Direction::Right => ans.push(Beam(Direction::Up)),
            },
            TileType::MirrorRight => match direction {
                Direction::Up => ans.push(Beam(Direction::Left)),
                Direction::Right => ans.push(Beam(Direction::Down)),
                Direction::Down => ans.push(Beam(Direction::Right)),
                Direction::Left => ans.push(Beam(Direction::Up)),
            },
            TileType::SplitterVertical => match direction {
                Direction::Up => ans.push(Beam(Direction::Up)),
                Direction::Down => ans.push(Beam(Direction::Down)),
                _ => ans.extend([Beam(Direction::Up), Beam(Direction::Down)]),
            },
            TileType::SplitterHorizontal => match direction {
                Direction::Left => ans.push(Beam(Direction::Left)),
                Direction::Right => ans.push(Beam(Direction::Right)),
                _ => ans.extend([Beam(Direction::Left), Beam(Direction::Right)]),
            },
        }
        ans
    }
    fn has_beam(&self, beam: &Beam) -> bool {
        self.1.contains(beam)
    }
    fn add_beam(&mut self, beam: Beam) {
        self.1.insert(beam);
    }
    fn remove_all_beams(&mut self) {
        self.1.clear();
    }
    fn is_energized(&self) -> bool {
        !self.1.is_empty()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beam(Direction);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
