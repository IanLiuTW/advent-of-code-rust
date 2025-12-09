use std::str::FromStr;

advent_of_code::solution!(2);

struct Present([u64; 3]);

impl FromStr for Present {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dims = [0u64; 3];
        let mut parts = s.split('x');

        for item in &mut dims {
            *item = parts
                .next()
                .ok_or("Missing dimension")?
                .parse()
                .map_err(|_| "Invalid number")?;
        }

        if parts.next().is_some() {
            return Err("Too many dimensions");
        }

        dims.sort_unstable();

        Ok(Present(dims))
    }
}

impl Present {
    fn surface_area_needed(&self) -> u64 {
        let [l, w, h] = self.0;

        (2 * l * w) + (2 * w * h) + (2 * h * l) + (l * w)
    }

    fn ribbon_needed(&self) -> u64 {
        let [l, w, h] = self.0;

        (2 * (l + w)) + (l * w * h)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let total_area: u64 = input
        .lines()
        .map(|line| line.parse::<Present>().expect("Malformed Input"))
        .map(|p| p.surface_area_needed())
        .sum();

    Some(total_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    let total_ribbon: u64 = input
        .lines()
        .map(|line| line.parse::<Present>().expect("Malformed Input"))
        .map(|p| p.ribbon_needed())
        .sum();

    Some(total_ribbon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(58));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
