use itertools::Itertools;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);

    let mut ans = 0;
    for (i, [lhs_data, rhs_data]) in input.iter().enumerate() {
        if lhs_data.cmp(rhs_data) == std::cmp::Ordering::Less {
            ans += i + 1;
        }
    }

    Some(ans as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = parse_input2(input);

    let dividers = vec![
        Data::List(vec![Data::List(vec![Data::Value(2)])]),
        Data::List(vec![Data::List(vec![Data::Value(6)])]),
    ];
    input.extend(dividers.clone());
    input.sort();

    let mut ans = 1;
    for (i, line) in input.iter().enumerate() {
        if dividers.contains(line) {
            ans *= i + 1;
        }
    }

    Some(ans as u32)
}

fn parse_input(input: &str) -> Vec<[Data; 2]> {
    input
        .split("\n\n")
        .map(|section| {
            let (line1, line2) = section.split_once("\n").unwrap();
            let data1: serde_json::Value = serde_json::from_str(line1).unwrap();
            let data2: serde_json::Value = serde_json::from_str(line2).unwrap();

            [Data::new(&data1), Data::new(&data2)]
        })
        .collect_vec()
}

fn parse_input2(input: &str) -> Vec<Data> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let data: serde_json::Value = serde_json::from_str(line).unwrap();
            Data::new(&data)
        })
        .collect_vec()
}

#[derive(Debug, Clone)]
enum Data {
    Value(u32),
    List(Vec<Data>),
}

impl Data {
    fn new(data: &serde_json::Value) -> Data {
        match data {
            serde_json::Value::Number(n) => Data::Value(n.as_u64().unwrap() as u32),
            serde_json::Value::Array(arr) => {
                let list = arr.iter().map(Data::new).collect();
                Data::List(list)
            }
            _ => unreachable!(),
        }
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn _compare_lists(lhs: &Vec<Data>, rhs: &Vec<Data>) -> std::cmp::Ordering {
            for pair in lhs.iter().zip_longest(rhs.iter()) {
                match pair {
                    itertools::EitherOrBoth::Both(lhs_data, rhs_data) => {
                        let result = lhs_data.cmp(rhs_data);
                        if result != std::cmp::Ordering::Equal {
                            return result;
                        }
                    }
                    itertools::EitherOrBoth::Left(_) => return std::cmp::Ordering::Greater,
                    itertools::EitherOrBoth::Right(_) => return std::cmp::Ordering::Less,
                }
            }

            std::cmp::Ordering::Equal
        }

        match (self, other) {
            (Data::Value(lhs), Data::Value(rhs)) => lhs.cmp(rhs),
            (Data::Value(lhs), Data::List(_)) => Data::List(vec![Data::Value(*lhs)]).cmp(other),
            (Data::List(_), Data::Value(rhs)) => self.cmp(&Data::List(vec![Data::Value(*rhs)])),
            (Data::List(lhs), Data::List(rhs)) => _compare_lists(lhs, rhs),
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Data {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }
}
