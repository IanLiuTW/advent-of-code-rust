advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<String> {
    let next = solve(input.trim());

    Some(next)
}

pub fn part_two(input: &str) -> Option<String> {
    let next = solve(input.trim());
    let next = solve(next.trim());

    Some(next)
}

struct Password {
    bytes: [u8; 8],
}

impl Password {
    fn new(input: &str) -> Self {
        let bytes: [u8; 8] = input
            .as_bytes()
            .try_into()
            .expect("Password must be exactly 8 bytes");
        Self { bytes }
    }

    fn increment(&mut self) {
        if let Some(idx) = self
            .bytes
            .iter()
            .position(|&b| matches!(b, b'i' | b'o' | b'l'))
        {
            self.bytes[idx] += 1;
            for b in &mut self.bytes[idx + 1..] {
                *b = b'a';
            }
            return;
        }

        for i in (0..8).rev() {
            if self.bytes[i] == b'z' {
                self.bytes[i] = b'a';
            } else {
                self.bytes[i] += 1;
                if matches!(self.bytes[i], b'i' | b'o' | b'l') {
                    self.bytes[i] += 1;
                    for b in &mut self.bytes[i + 1..] {
                        *b = b'a';
                    }
                }
                return;
            }
        }
    }

    fn has_straight(&self) -> bool {
        self.bytes
            .windows(3)
            .any(|w| w[1] == w[0] + 1 && w[2] == w[1] + 1)
    }

    fn has_two_pairs(&self) -> bool {
        let mut pairs = 0;
        let mut i = 0;

        while i < self.bytes.len() - 1 {
            if self.bytes[i] == self.bytes[i + 1] {
                pairs += 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        pairs >= 2
    }

    fn is_valid(&self) -> bool {
        self.has_straight() && self.has_two_pairs()
    }

    fn to_string(&self) -> String {
        String::from_utf8(self.bytes.to_vec()).unwrap()
    }
}

pub fn solve(input: &str) -> String {
    let mut pwd = Password::new(input);

    loop {
        pwd.increment();
        if pwd.is_valid() {
            return pwd.to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("vzbxxyzz".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("vzcaabcc".to_string()));
    }
}
