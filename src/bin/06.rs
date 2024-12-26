advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = input.lines();
    let time = get_iter(input.next().unwrap());
    let distance = get_iter(input.next().unwrap());

    let mut ans = 1;
    for (t, d) in std::iter::zip(time, distance) {
        let t = t.parse().unwrap();
        let d = d.parse().unwrap();
        ans *= (t - 1) - (bs(0, t, d, t) - 1) * 2;
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = input.lines();
    let time = get_iter(input.next().unwrap()).collect::<String>().parse::<u64>().unwrap();
    let distance = get_iter(input.next().unwrap()).collect::<String>().parse::<u64>().unwrap();
    let ans = (time - 1) - (bs(0, time, distance, time) - 1) * 2;
    Some(ans)
}

fn get_iter(line: &str) -> impl Iterator<Item = &str> {
    line.split_once(':').unwrap().1.split_ascii_whitespace()
}

fn bs(mut lo: u64, mut hi: u64, distance: u64, time: u64) -> u64 {
    while lo < hi {
        let mid = (lo + hi) / 2;
        if (time - mid) * mid > distance {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
