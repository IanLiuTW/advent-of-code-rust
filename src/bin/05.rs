use std::str::{Lines, SplitAsciiWhitespace};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut input: Lines<'_> = input.lines();

    let seeds = get_seeds(input.next().unwrap()).map(|x| x.parse::<u64>().unwrap());
    input.next();
    let maps = get_maps(&mut input);

    Some(seeds.map(|seed| get_final_val(seed, &maps)).min().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input: Lines<'_> = input.lines();

    let mut seeds = get_seeds(input.next().unwrap()).map(|x| x.parse::<u64>().unwrap());
    input.next();
    let maps = get_maps(&mut input);

    let mut ans = std::u64::MAX;
    loop {
        let seed_start = match seeds.next() {
            Some(num) => num,
            None => break,
        };
        let seed_range = seeds.next().unwrap();
        for seed in seed_start..seed_start + seed_range {
            ans = ans.min(get_final_val(seed, &maps));
        }
    }
    Some(ans)
}

#[derive(Debug)]
struct Entry {
    start: u64,
    end: u64,
    dest_start: u64,
}

fn get_seeds(line: &str) -> SplitAsciiWhitespace<'_> {
    line.split_once(':').unwrap().1.split_ascii_whitespace()
}

fn get_maps(input: &mut Lines<'_>) -> Vec<Vec<Entry>> {
    let mut maps = Vec::new();

    let mut map: Vec<_> = Vec::new();
    let mut skip_line = true;
    for line in input {
        if line.is_empty() {
            maps.push(map);

            map = Vec::new();
            skip_line = true;
        } else if skip_line {
            skip_line = false;
            continue;
        } else {
            let mut line = line.splitn(3, ' ').map(|x| x.parse::<u64>().unwrap());
            let des = line.next().unwrap();
            let src = line.next().unwrap();
            let range = line.next().unwrap();
            map.push(Entry {
                start: src,
                end: src + range - 1,
                dest_start: des,
            });
        }
    }
    maps.push(map);
    maps
}

fn get_final_val(mut num: u64, maps: &Vec<Vec<Entry>>) -> u64 {
    'maps: for map in maps {
        for entry in map {
            if entry.start <= num && num <= entry.end {
                num = entry.dest_start + num - entry.start;
                continue 'maps;
            }
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
