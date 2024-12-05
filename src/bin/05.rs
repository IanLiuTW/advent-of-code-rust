use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans: u32 = 0;

    let mut lines = input.lines().peekable();
    let graph = get_graph(&mut lines);

    for line in lines {
        let orig_order: Vec<u32> = line
            .split(',')
            .map(|item| item.parse::<u32>().unwrap())
            .collect();

        let nums_set: HashSet<u32> = orig_order.iter().cloned().collect();

        let topo_order = get_topo_order(&graph, &nums_set);

        if orig_order == topo_order {
            let mid = topo_order.len() / 2;
            ans += topo_order[mid];
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans: u32 = 0;

    let mut lines = input.lines().peekable();
    let graph = get_graph(&mut lines);

    for line in lines {
        let orig_order: Vec<u32> = line
            .split(',')
            .map(|item| item.parse::<u32>().unwrap())
            .collect();

        let nums_set: HashSet<u32> = orig_order.iter().cloned().collect();

        let topo_order = get_topo_order(&graph, &nums_set);

        if orig_order != topo_order {
            let mid = topo_order.len() / 2;
            ans += topo_order[mid];
        }
    }

    Some(ans)
}

fn get_graph<'a>(lines: &mut impl Iterator<Item = &'a str>) -> HashMap<u32, Vec<u32>> {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        let mut nums = line.split('|');
        let left = nums.next().unwrap().parse::<u32>().unwrap();
        let right = nums.next().unwrap().parse::<u32>().unwrap();

        graph.entry(left).or_default().push(right);
        graph.entry(right).or_default();
    }

    graph
}

fn get_topo_order(graph: &HashMap<u32, Vec<u32>>, nums_set: &HashSet<u32>) -> Vec<u32> {
    fn get_indegree(graph: &HashMap<u32, Vec<u32>>, nums_set: &HashSet<u32>) -> HashMap<u32, u32> {
        let mut indegree: HashMap<u32, u32> = HashMap::new();
        for prev_num in nums_set {
            if !nums_set.contains(prev_num) {
                continue;
            }
            for next_num in graph.get(prev_num).unwrap() {
                if !nums_set.contains(next_num) {
                    continue;
                }
                *indegree.entry(*next_num).or_default() += 1;
            }
        }
        indegree
    }

    let mut indegree = get_indegree(graph, nums_set);
    let mut topo_order: Vec<u32> = nums_set
        .iter()
        .filter(|num| !indegree.contains_key(num))
        .cloned()
        .collect();

    let mut i = 0;
    while i < topo_order.len() {
        let num = topo_order[i];
        if let Some(next_nums) = graph.get(&num) {
            for &next_num in next_nums {
                if let Some(next_num_ind) = indegree.get_mut(&next_num) {
                    *next_num_ind -= 1;
                    if *next_num_ind == 0 {
                        topo_order.push(next_num);
                    }
                }
            }
        }
        i += 1;
    }
    topo_order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
