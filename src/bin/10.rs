use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = 1;

    let map = Map::new(input);
    let starting_node = map.get_starting_node();
    let starting_neighbors = map.get_neighbors(&starting_node);
    let (mut n1, mut p1) = (starting_neighbors[0].clone(), starting_node.clone());
    let (mut n2, mut p2) = (starting_neighbors[1].clone(), starting_node.clone());
    while n1 != n2 {
        (n1, p1) = map.go_forward(n1, p1);
        (n2, p2) = map.go_forward(n2, p2);
        ans += 1;
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut nodes = HashSet::new();

    let map = Map::new(input);
    let starting_node = map.get_starting_node();
    nodes.insert(starting_node.get_tuple());

    let starting_neighbors = map.get_neighbors(&starting_node);
    let (mut n1, mut p1) = (starting_neighbors[0].clone(), starting_node.clone());
    let (mut n2, mut p2) = (starting_neighbors[1].clone(), starting_node.clone());
    nodes.insert(n1.get_tuple());
    nodes.insert(n2.get_tuple());

    while n1 != n2 {
        (n1, p1) = map.go_forward(n1, p1);
        (n2, p2) = map.go_forward(n2, p2);
        nodes.insert(n1.get_tuple());
        nodes.insert(n2.get_tuple());
    }

    let mut ans = 0;
    for (i, line) in input.lines().enumerate() {
        let mut is_inside = false;
        for (j, c) in line.chars().enumerate() {
            match nodes.contains(&(i, j)) {
                true => match c {
                    '|' | 'J' | 'L' => is_inside = !is_inside,
                    'S' if map.is_connected_upward(&starting_node) => is_inside = !is_inside,
                    _ => {}
                },
                false if is_inside => ans += 1,
                _ => {}
            }
        }
    }
    Some(ans)
}

#[derive(Debug, Clone, Copy)]
struct Direction(isize, isize);
impl Direction {
    fn up() -> Self {
        Self(-1, 0)
    }
    fn down() -> Self {
        Self(1, 0)
    }
    fn left() -> Self {
        Self(0, -1)
    }
    fn right() -> Self {
        Self(0, 1)
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Node(usize, usize);
impl Node {
    fn go_to(&self, direction: &Direction) -> Node {
        Self(
            (self.0 as isize + direction.0) as usize,
            (self.1 as isize + direction.1) as usize,
        )
    }

    fn get_tuple(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}

struct Map {
    map: Vec<Vec<char>>,
    m: usize,
    n: usize,
}
impl Map {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
        let m = map.len();
        let n = map.first().unwrap().len();
        Self { map, m, n }
    }

    fn get_starting_node(&self) -> Node {
        for (i, row) in self.map.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if *char == 'S' {
                    return Node(i, j);
                }
            }
        }
        Node(0, 0)
    }

    fn is_connected(&self, node: &Node, direction: &Direction) -> bool {
        let other = node.go_to(direction);
        match self.get_symbol(&other) {
            '.' => false,
            _ => self.get_neighbors(&other).contains(node),
        }
    }

    fn is_connected_upward(&self, node: &Node) -> bool {
        if node.0 == 0 {
            return false;
        }
        self.is_connected(node, &Direction::up())
    }

    fn get_symbol(&self, node: &Node) -> char {
        self.map[node.0][node.1]
    }

    fn get_directions(&self, node: &Node) -> [Direction; 2] {
        match self.get_symbol(node) {
            '|' => [Direction::up(), Direction::down()],
            '-' => [Direction::left(), Direction::right()],
            'L' => [Direction::up(), Direction::right()],
            'J' => [Direction::up(), Direction::left()],
            '7' => [Direction::left(), Direction::down()],
            'F' => [Direction::right(), Direction::down()],
            'S' => {
                let mut directions: Vec<Direction> = vec![];
                for (idx, idx_bound, direction) in [
                    (node.0, 0, Direction::up()),
                    (node.0, self.m - 1, Direction::down()),
                    (node.1, 0, Direction::left()),
                    (node.1, self.n - 1, Direction::right()),
                ] {
                    if idx != idx_bound && self.is_connected(node, &direction) {
                        directions.push(direction);
                    }
                }
                directions.as_slice().try_into().unwrap()
            }
            _ => unreachable!(),
        }
    }

    fn get_neighbors(&self, node: &Node) -> [Node; 2] {
        let directions = self.get_directions(node);
        [node.go_to(&directions[0]), node.go_to(&directions[1])]
    }

    fn go_forward(&self, node: Node, prev: Node) -> (Node, Node) {
        let neighbors = self.get_neighbors(&node);
        (
            neighbors.iter().find(|&n| n != &prev).unwrap().clone(),
            node,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
