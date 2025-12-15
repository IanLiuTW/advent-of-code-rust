use std::collections::{HashMap, HashSet};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse_input(input);

    let n = graph.len();
    let mut visited: HashSet<&str> = HashSet::with_capacity(n);

    let result = graph.keys().map(|&city| {}).min();

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> HashMap<&str, HashMap<&str, u32>> {
    let mut graph: HashMap<&str, HashMap<&str, u32>> = HashMap::new();

    for line in input.lines() {
        let (cities, distance) = line.split_once(" = ").expect("Malformed line");
        let (city1, city2) = cities.split_once(" to ").expect("Malformed line");

        let distance = distance.parse::<u32>().expect("Malformed distance");
        graph.entry(city1).or_default().insert(city2, distance);
        graph.entry(city2).or_default().insert(city1, distance);
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
