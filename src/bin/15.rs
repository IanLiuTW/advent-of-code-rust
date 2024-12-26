use std::collections::VecDeque;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = 0;
    let input = input.split(',');
    for s in input {
        ans += get_hash(s);
    }
    Some(ans as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.split(',');
    const EMPTY_VECDEQUE: VecDeque<Lens> = VecDeque::new();
    let mut boxes = [EMPTY_VECDEQUE; 256];
    for item in input {
        match item.chars().last().unwrap() {
            '-' => box_del(&mut boxes, item),
            _ => box_put(&mut boxes, item),
        }
    }

    let ans = boxes
        .iter()
        .enumerate()
        .map(|(box_i, box_)| {
            box_.iter()
                .enumerate()
                .map(|(i, lens)| (box_i + 1) as u32 * (i + 1) as u32 * lens.1)
                .sum::<u32>()
        })
        .sum::<u32>();
    Some(ans)
}

fn get_ascii_val(c: &char) -> u32 {
    *c as u32
}

fn get_hash(s: &str) -> usize {
    let mut ans = 0;
    for c in s.chars() {
        ans = (ans + get_ascii_val(&c)) * 17 % 256;
    }
    ans as usize
}

#[derive(Debug)]
struct Lens(String, u32);

fn box_put(boxes: &mut [VecDeque<Lens>], item: &str) {
    let item = item.split_once('=').unwrap();
    let item = Lens(item.0.to_string(), item.1.parse().unwrap());
    let hash = get_hash(&item.0);

    if let Some(i) = box_position(&boxes[hash], &item.0) {
        boxes[hash][i] = item;
    } else {
        boxes[hash].push_back(item);
    }
}

fn box_del(boxes: &mut [VecDeque<Lens>], item: &str) {
    let label = item.split_once('-').unwrap().0;
    let hash = get_hash(label);

    if let Some(i) = box_position(&boxes[hash], label) {
        boxes[hash].remove(i);
    }
}

fn box_position(box_: &VecDeque<Lens>, label: &str) -> Option<usize> {
    box_.iter().position(|lens| lens.0 == label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }

    #[test]
    fn test_s() {
        let s = "rn=1";
        assert_eq!(get_hash(s), 30);
        let s = "cm-";
        assert_eq!(get_hash(s), 253);
        let s = "rn";
        assert_eq!(get_hash(s), 0);
        let s = "qp";
        assert_eq!(get_hash(s), 1);
        let s = "ot";
        assert_eq!(get_hash(s), 3);
    }
}
