use regex::Regex;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = 0;

    for line in parse_input(input) {
        let (pair1, pair2) = (line[0], line[1]);

        if has_inside_relationship(&pair1, &pair2) {
            ans += 1;
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans = 0;

    for line in parse_input(input) {
        let (pair1, pair2) = (line[0], line[1]);

        if has_overlap_relationship(&pair1, &pair2) {
            ans += 1;
        }
    }

    Some(ans)
}

fn parse_input(input: &str) -> Vec<[[u32; 2]; 2]> {
    let re_line = Regex::new("(.*)-(.*),(.*)-(.*)").unwrap();

    input
        .lines()
        .map(|line| {
            let line = re_line.captures(line).unwrap();

            [
                [line[1].parse().unwrap(), line[2].parse().unwrap()],
                [line[3].parse().unwrap(), line[4].parse().unwrap()],
            ]
        })
        .collect::<Vec<[[u32; 2]; 2]>>()
}

fn has_inside_relationship(pair1: &[u32; 2], pair2: &[u32; 2]) -> bool {
    if pair1[1] - pair1[0] > pair2[1] - pair2[0] {
        return has_inside_relationship(pair2, pair1);
    }

    pair2[0] <= pair1[0] && pair1[1] <= pair2[1]
}

fn has_overlap_relationship(pair1: &[u32; 2], pair2: &[u32; 2]) -> bool {
    if pair1[1] > pair2[1] {
        return has_overlap_relationship(pair2, pair1);
    }

    pair2[0] <= pair1[1] && pair1[1] <= pair2[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
