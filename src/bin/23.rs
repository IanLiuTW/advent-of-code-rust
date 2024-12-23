use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);

    let mut ans = 0;

    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for [item1, item2] in input {
        graph.entry(item1).or_default().insert(item2);
        graph.entry(item2).or_default().insert(item1);

        let common_nodes = graph
            .get(item1)
            .unwrap()
            .intersection(graph.get(item2).unwrap());

        for item3 in common_nodes {
            for item in [item1, item2, item3] {
                if item.starts_with('t') {
                    ans += 1;
                    break;
                }
            }
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<String> {
    let input = parse_input(input);

    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for [item1, item2] in input {
        graph.entry(item1).or_default().insert(item2);
        graph.entry(item2).or_default().insert(item1);
    }

    let mut ans = vec![];

    for node in graph.keys() {
        let mut longest = vec![];

        bt(&graph, node, &mut vec![], &mut HashSet::new(), &mut longest);

        if longest.len() > ans.len() {
            ans = longest;
        }
    }

    ans.sort();

    Some(ans.iter().join(","))
}

fn bt<'a>(
    graph: &'a HashMap<&'a str, HashSet<&'a str>>,
    node: &'a str,
    combo: &mut Vec<&'a str>,
    _visited: &mut HashSet<&'a str>,
    _longest: &mut Vec<&'a str>,
) {
    _visited.insert(node);

    for n in combo.iter() {
        if !graph.get(node).unwrap().contains(n) {
            return;
        }
    }

    combo.push(node);

    for n in graph.get(node).unwrap().iter() {
        if _visited.contains(n) {
            continue;
        }

        bt(graph, n, combo, _visited, _longest);
    }

    if combo.len() > _longest.len() {
        *_longest = combo.clone();
    }

    combo.pop();
}

fn parse_input(input: &str) -> Vec<[&str; 2]> {
    input
        .lines()
        .map(|line| {
            let line = line.split_once('-').unwrap();
            [line.0, line.1]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
