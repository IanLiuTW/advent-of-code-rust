use std::collections::LinkedList;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let sequence = convert_input_to_sequence(input);
    let compressed_sequence = compress_sequence(sequence);
    let check_sum = get_check_sum(compressed_sequence);

    Some(check_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sequence = convert_input_to_sequence(input);
    let compressed_sequence = compress_sequence2(sequence);
    let check_sum = get_check_sum(compressed_sequence);

    Some(check_sum)
}

fn convert_input_to_sequence(input: &str) -> Vec<i32> {
    let input = input.lines().next().unwrap();

    let mut sequence = vec![];
    let mut is_file = true;
    let mut id = 0;

    for num in input.chars() {
        let num = num.to_digit(10).unwrap() as usize;
        if is_file {
            sequence.extend(vec![id; num]);
            id += 1;
        } else {
            sequence.extend(vec![-1; num]);
        }
        is_file = !is_file;
    }

    sequence
}

fn compress_sequence(mut sequence: Vec<i32>) -> Vec<i32> {
    let mut right = sequence.iter().rposition(|&x| x != -1).unwrap();
    for left in 0..sequence.len() {
        if left >= right {
            break;
        }

        let num = sequence[left];
        if num == -1 {
            sequence[left] = sequence[right];
            sequence[right] = -1;
            right -= 1;
            while sequence[right] == -1 {
                right -= 1;
            }
        }
    }

    sequence
}

fn get_check_sum(sequence: Vec<i32>) -> u64 {
    sequence
        .iter()
        .enumerate()
        .map(
            |(i, &num)| {
                if num == -1 {
                    0
                } else {
                    i as u64 * num as u64
                }
            },
        )
        .sum()
}

fn get_sections(sequence: &Vec<i32>) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut file_sections = vec![];
    let mut free_sections = vec![];

    let mut i = 0;
    while i < sequence.len() {
        let section_type = sequence[i];

        let mut j = i;
        while j < sequence.len() && sequence[j] == section_type {
            j += 1;
        }

        if section_type != -1 {
            file_sections.push((i, j - i));
        } else {
            free_sections.push((i, j - i));
        }

        i = j;
    }

    (file_sections, free_sections)
}

fn compress_sequence2(mut sequence: Vec<i32>) -> Vec<i32> {
    let (mut file_sections, mut free_sections) = get_sections(&sequence);

    while let Some((file_idx, file_len)) = file_sections.pop() {
        for item in free_sections.iter_mut() {
            if item.0 > file_idx {
                continue;
            }
            if item.1 >= file_len {
                for delta in 0..file_len {
                    sequence[item.0 + delta] = sequence[file_idx];
                }
                for delta in 0..file_len {
                    sequence[file_idx + delta] = -1;
                }

                *item = (item.0 + file_len, item.1 - file_len);
                break;
            }
        }
    }

    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let sequence =
            convert_input_to_sequence(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            sequence,
            vec![
                0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5,
                5, 5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9
            ]
        );
        let compressed_sequence = compress_sequence(sequence);
        assert_eq!(
            compressed_sequence,
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
            ]
        );
        let check_sum = get_check_sum(compressed_sequence);
        assert_eq!(check_sum, 1928);

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let sequence =
            convert_input_to_sequence(&advent_of_code::template::read_file("examples", DAY));
        let (file_sections, free_sections) = get_sections(&sequence);
        assert_eq!(
            file_sections,
            vec![
                (0, 2),
                (5, 3),
                (11, 1),
                (15, 3),
                (19, 2),
                (22, 4),
                (27, 4),
                (32, 3),
                (36, 4),
                (40, 2)
            ]
        );
        assert_eq!(
            free_sections,
            vec![
                (2, 3,),
                (8, 3,),
                (12, 3,),
                (18, 1,),
                (21, 1,),
                (26, 1,),
                (31, 1,),
                (35, 1,),
            ]
        );
        let compressed_sequence = compress_sequence2(sequence);
        assert_eq!(
            compressed_sequence,
            vec![
                0, 0, 9, 9, 2, 1, 1, 1, 7, 7, 7, -1, 4, 4, -1, 3, 3, 3, -1, -1, -1, -1, 5, 5, 5, 5,
                -1, 6, 6, 6, 6, -1, -1, -1, -1, -1, 8, 8, 8, 8, -1, -1,
            ]
        );
        let check_sum = get_check_sum(compressed_sequence);
        assert_eq!(check_sum, 2858);

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
