use std::collections::HashMap;

advent_of_code::solution!(11);

const YOU_NODE: &str = "you";
const END_NODE: &str = "out";
const SVR_NODE: &str = "svr";
const PROBLEM_NODES: [&str; 2] = ["dac", "fft"];

type NodeId = u32;

pub fn part_one(input: &str) -> Option<u64> {
    let (graph, mapping) = parse_input(input)?;

    let start = *mapping.get(YOU_NODE)?;
    let end = *mapping.get(END_NODE)?;

    let mut visited = vec![false; graph.len()];
    Some(count_paths(&graph, &mut visited, start, end))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (graph, mapping) = parse_input(input)?;

    let start = *mapping.get(SVR_NODE)?;
    let end = *mapping.get(END_NODE)?;
    let problem_ids: Vec<NodeId> = PROBLEM_NODES
        .iter()
        .filter_map(|n| mapping.get(*n).copied())
        .collect();

    let mut memo = HashMap::new();
    Some(count_paths_memo(
        &graph,
        &problem_ids,
        &mut memo,
        start,
        end,
        0,
    ))
}

fn parse_input(input: &str) -> Option<(Vec<Vec<NodeId>>, HashMap<&str, NodeId>)> {
    let mut mapping: HashMap<&str, NodeId> = HashMap::new();
    let mut next_id = 0u32;
    let mut raw_edges = Vec::new();

    for line in input.lines() {
        let (src_str, rest) = line.split_once(": ")?;

        let src_id = if let Some(&id) = mapping.get(src_str) {
            id
        } else {
            let id = next_id;
            mapping.insert(src_str, id);
            next_id += 1;
            id
        };

        for dst_str in rest.split_ascii_whitespace() {
            let dst_id = if let Some(&id) = mapping.get(dst_str) {
                id
            } else {
                let id = next_id;
                mapping.insert(dst_str, id);
                next_id += 1;
                id
            };
            raw_edges.push((src_id, dst_id));
        }
    }

    let mut adj = vec![Vec::new(); next_id as usize];
    for (u, v) in raw_edges {
        adj[u as usize].push(v);
    }

    Some((adj, mapping))
}

fn count_paths(
    graph: &[Vec<NodeId>],
    visited: &mut Vec<bool>,
    current: NodeId,
    target: NodeId,
) -> u64 {
    if current == target {
        return 1;
    }

    visited[current as usize] = true;
    let mut count = 0;

    if let Some(neighbors) = graph.get(current as usize) {
        for &next in neighbors {
            if !visited[next as usize] {
                count += count_paths(graph, visited, next, target);
            }
        }
    }

    visited[current as usize] = false;
    count
}

fn count_paths_memo(
    graph: &[Vec<NodeId>],
    problem_nodes: &[NodeId],
    memo: &mut HashMap<(NodeId, u8), u64>,
    current: NodeId,
    target: NodeId,
    passed_problem_count: u8,
) -> u64 {
    if current == target {
        return if passed_problem_count as usize == problem_nodes.len() {
            1
        } else {
            0
        };
    }

    let key = (current, passed_problem_count);
    if let Some(&res) = memo.get(&key) {
        return res;
    }

    let new_passed_problem_count = if problem_nodes.contains(&current) {
        passed_problem_count + 1
    } else {
        passed_problem_count
    };

    let mut count = 0;
    if let Some(neighbors) = graph.get(current as usize) {
        for &next in neighbors {
            count += count_paths_memo(
                graph,
                problem_nodes,
                memo,
                next,
                target,
                new_passed_problem_count,
            );
        }
    }

    memo.insert(key, count);
    count
}

mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let data = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let result = part_two(data);
        assert_eq!(result, Some(2));
    }
}
