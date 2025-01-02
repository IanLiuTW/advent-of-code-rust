use std::{
    borrow::Borrow,
    collections::{HashSet, VecDeque},
    fmt::Debug,
};

use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut simulation = Simulation::new(input);

    simulation.run(20);

    let mut inspection_times = simulation.get_monkey_inspections_times();
    Some(inspection_times[0] * inspection_times[1])
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut simulation = Simulation::new(input);
    simulation.update_mod_val_to_super_mod();

    simulation.run(10000);

    let mut inspection_times = simulation.get_monkey_inspections_times();
    Some(inspection_times[0] * inspection_times[1])
}

struct Monkey {
    items: VecDeque<u64>,
    inspect: Box<dyn Fn(u64) -> u64>,
    test_divisible_by: u64,
    true_subject: usize,
    false_subject: usize,
    inspection_times: u64,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .finish()
    }
}

impl Monkey {
    fn new(
        items: VecDeque<u64>,
        inspect: Box<dyn Fn(u64) -> u64>,
        test_divisible_by: u64,
        true_subject: usize,
        false_subject: usize,
    ) -> Self {
        Self {
            items,
            inspect,
            test_divisible_by,
            true_subject,
            false_subject,
            inspection_times: 0,
        }
    }

    fn process_an_item(&mut self, mod_val: Option<u64>) -> Option<(usize, u64)> {
        let item = self.items.pop_front()?;

        let mut worry_level = self.inspect(item);
        worry_level = self.normalize_worry_level(worry_level, mod_val);

        self.get_test_outcome(worry_level)
    }

    fn inspect(&mut self, item: u64) -> u64 {
        self.inspection_times += 1;

        (self.inspect.borrow() as &dyn Fn(u64) -> u64)(item)
    }

    fn normalize_worry_level(&self, worry_level: u64, mod_val: Option<u64>) -> u64 {
        if let Some(mod_val) = mod_val {
            worry_level % mod_val
        } else {
            worry_level / 3
        }
    }

    fn get_test_outcome(&self, worry_level: u64) -> Option<(usize, u64)> {
        if worry_level % self.test_divisible_by == 0 {
            Some((self.true_subject, worry_level))
        } else {
            Some((self.false_subject, worry_level))
        }
    }

    fn add_item(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

#[derive(Debug)]
struct Simulation {
    monkeys: Vec<Monkey>,
    mod_val: Option<u64>,
}

impl Simulation {
    fn new(input: &str) -> Self {
        let monkeys = input
            .split("\n\n")
            .map(|lines| {
                let lines = lines.lines().collect_vec();

                let _ = lines[0].trim_start_matches("Monkey ").trim_end_matches(":");

                let items = lines[1]
                    .trim_start_matches("  Starting items: ")
                    .split(", ")
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect::<VecDeque<u64>>();

                let (op, num) = lines[2]
                    .trim_start_matches("  Operation: new = old ")
                    .split_once(" ")
                    .unwrap();
                let op: &'static dyn Fn(u64, u64) -> u64 = match op {
                    "+" => &|a: u64, b: u64| a + b,
                    "*" => &|a: u64, b: u64| a * b,
                    _ => unreachable!(),
                };
                let operation = match num {
                    "old" => Box::new(|a: u64| op(a, a)) as Box<dyn Fn(u64) -> u64>,
                    _ => {
                        let num = num.parse::<u64>().unwrap();
                        Box::new(move |a: u64| op(a, num)) as Box<dyn Fn(u64) -> u64>
                    }
                };

                let test_divisible_by = lines[3]
                    .trim_start_matches("  Test: divisible by ")
                    .parse::<u64>()
                    .unwrap();

                let true_subject = lines[4]
                    .trim_start_matches("    If true: throw to monkey ")
                    .parse::<usize>()
                    .unwrap();

                let false_subject = lines[5]
                    .trim_start_matches("    If false: throw to monkey ")
                    .parse::<usize>()
                    .unwrap();

                Monkey::new(
                    items,
                    operation,
                    test_divisible_by,
                    true_subject,
                    false_subject,
                )
            })
            .collect_vec();

        Simulation {
            monkeys,
            mod_val: None,
        }
    }

    fn run(&mut self, times: usize) {
        for _ in 0..times {
            for i in 0..self.monkeys.len() {
                while let Some((idx, item)) = self.monkeys[i].process_an_item(self.mod_val) {
                    self.monkeys[idx].add_item(item);
                }
            }
        }
    }

    fn get_monkey_inspections_times(&self) -> Vec<u64> {
        let mut inspection_times = self
            .monkeys
            .iter()
            .map(|monkey| monkey.inspection_times)
            .collect_vec();

        inspection_times.sort();
        inspection_times.reverse();

        inspection_times
    }

    fn update_mod_val_to_super_mod(&mut self) {
        let test_vals = self
            .monkeys
            .iter()
            .map(|monkey| monkey.test_divisible_by)
            .collect::<HashSet<u64>>();

        self.mod_val = Some(test_vals.into_iter().reduce(|accu, x| accu * x).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2713310158));
    }
}
