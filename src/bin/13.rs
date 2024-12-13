use regex::Regex;

advent_of_code::solution!(13);

struct Data {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input, 0);
    Some(count_tokens(input))
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input, 10000000000000);
    Some(count_tokens(input))
}

fn parse_input(input: &str, delta: i64) -> Vec<Data> {
    let button_re = Regex::new(r"X\+(\d*).*Y\+(\d*)").unwrap();
    let prize_re = Regex::new(r"X\=(\d*).*Y\=(\d*)").unwrap();

    let mut input = input.lines().peekable();
    let mut parsed_input = vec![];

    while input.peek().is_some() {
        let button_a = input.next().unwrap();
        let button_a = button_re.captures(button_a).unwrap();
        let button_b = input.next().unwrap();
        let button_b = button_re.captures(button_b).unwrap();
        let prize = input.next().unwrap();
        let prize = prize_re.captures(prize).unwrap();
        input.next();

        parsed_input.push(Data {
            ax: button_a[1].parse().unwrap(),
            ay: button_a[2].parse().unwrap(),
            bx: button_b[1].parse().unwrap(),
            by: button_b[2].parse().unwrap(),
            px: prize[1].parse::<i64>().unwrap() + delta,
            py: prize[2].parse::<i64>().unwrap() + delta,
        })
    }

    parsed_input
}

fn count_tokens(input: Vec<Data>) -> u64 {
    let mut ans = 0;

    for data in input {
        let w = data.ax * data.by - data.bx * data.ay;
        let wa = data.px * data.by - data.py * data.bx;
        let wb = data.py * data.ax - data.px * data.ay;

        if w != 0 {
            let wa = wa / w;
            let wb = wb / w;

            for a in wa - 10..wa + 10 {
                for b in wb - 10..wb + 10 {
                    let x = a * data.ax + b * data.bx;
                    let y = a * data.ay + b * data.by;

                    if x == data.px && y == data.py {
                        ans += 3 * a + b;
                    }
                }
            }
        }
    }

    ans as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
