use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    let mut ans = 0;

    for line in input.lines() {
        let secret = line.parse::<u64>().unwrap();
        let mut secret = Secret::new(secret);

        for _ in 0..2000 {
            secret.go_to_next_secret();
        }

        ans += secret.secret;
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total_selling_mapping: HashMap<(i8, i8, i8, i8), u64> = HashMap::new();

    for line in input.lines() {
        let secret = line.parse::<u64>().unwrap();
        let secret = Secret::new(secret);

        let selling_mapping = get_selling_mapping(secret);
        for (key, val) in selling_mapping {
            *total_selling_mapping.entry(key).or_default() += val;
        }
    }

    Some(*total_selling_mapping.values().max().unwrap())
}

fn get_selling_mapping(mut secret: Secret) -> HashMap<(i8, i8, i8, i8), u64> {
    let mut selling_mapping: HashMap<(i8, i8, i8, i8), u64> = HashMap::new();

    let mut changes = VecDeque::new();
    let mut last_digit = secret.last_digit();

    for _ in 0..2000 {
        secret.go_to_next_secret();
        changes.push_back(secret.last_digit() as i8 - last_digit as i8);
        last_digit = secret.last_digit();

        while changes.len() > 4 {
            changes.pop_front();
        }

        if changes.len() == 4 {
            let key = (changes[0], changes[1], changes[2], changes[3]);
            selling_mapping.entry(key).or_insert(last_digit);
        }
    }

    selling_mapping
}

#[derive(Clone, Copy)]
struct Secret {
    secret: u64,
}

impl Secret {
    fn new(secret: u64) -> Self {
        Secret { secret }
    }

    fn last_digit(&self) -> u64 {
        self.secret % 10
    }

    fn go_to_next_secret(&mut self) {
        self._mix(self.secret * 64);
        self._prune();

        self._mix(self.secret / 32);
        self._prune();

        self._mix(self.secret * 2048);
        self._prune();
    }

    fn _mix(&mut self, val: u64) {
        self.secret ^= val
    }

    fn _prune(&mut self) {
        self.secret %= 16777216
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut secret = Secret::new(42);
        secret._mix(15);
        assert_eq!(secret.secret, 37);
        let mut secret = Secret::new(100000000);
        secret._prune();
        assert_eq!(secret.secret, 16113920);

        let mut secret = Secret::new(123);
        secret.go_to_next_secret();
        assert_eq!(secret.secret, 15887950);
        secret.go_to_next_secret();
        assert_eq!(secret.secret, 16495136);
        secret.go_to_next_secret();
        assert_eq!(secret.secret, 527345);

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let input = "1\n2\n3\n2024";
        let result = part_two(input);
        assert_eq!(result, Some(23));
    }
}
