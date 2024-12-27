use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(20);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
enum Module {
    FlipFlop { switch: bool },
    Conjunction { high_pulses: HashSet<String> },
    BroadCaster,
    Dummy,
}

impl Module {
    fn new_flip_flop() -> Self {
        Module::FlipFlop { switch: false }
    }

    fn new_conjunction() -> Self {
        Module::Conjunction {
            high_pulses: HashSet::new(),
        }
    }

    fn new_broadcaster() -> Self {
        Module::BroadCaster
    }

    fn new_dummy() -> Self {
        Module::Dummy
    }

    fn run(
        &mut self,
        from_module_id: String,
        pulse: Pulse,
        sources: &HashSet<String>,
    ) -> Option<Pulse> {
        match self {
            Module::FlipFlop { switch } => {
                if pulse == Pulse::High {
                    return None;
                }

                *switch = !*switch;
                if *switch {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            }
            Module::Conjunction { high_pulses } => match pulse {
                Pulse::High => {
                    high_pulses.insert(from_module_id.to_string());
                    if sources.len() == high_pulses.len() {
                        Some(Pulse::Low)
                    } else {
                        Some(Pulse::High)
                    }
                }
                Pulse::Low => {
                    high_pulses.remove(&from_module_id);
                    Some(Pulse::High)
                }
            },
            Module::BroadCaster => Some(pulse),
            Module::Dummy => None,
        }
    }
}

#[derive(Debug)]
struct Device {
    modules: HashMap<String, Module>,
    sources: HashMap<String, HashSet<String>>,
    destinations: HashMap<String, HashSet<String>>,
    pulses: VecDeque<(String, Pulse, String)>,
    low_pulse_cnt: u32,
    high_pulse_cnt: u32,
}

impl Device {
    fn new() -> Self {
        Device {
            modules: HashMap::new(),
            sources: HashMap::new(),
            destinations: HashMap::new(),
            pulses: VecDeque::new(),
            low_pulse_cnt: 0,
            high_pulse_cnt: 0,
        }
    }

    fn register_module(
        &mut self,
        id: String,
        module: Module,
        destinations: Vec<String>,
    ) -> &mut Module {
        let module = self.modules.entry(id.to_string()).or_insert(module);

        for destination in destinations {
            self.destinations
                .entry(id.to_string())
                .or_default()
                .insert(destination.to_string());

            self.sources
                .entry(destination.to_string())
                .or_default()
                .insert(id.to_string());
        }

        module
    }

    fn update_pulse_cnt(&mut self, pulse: Pulse) {
        match pulse {
            Pulse::High => self.high_pulse_cnt += 1,
            Pulse::Low => self.low_pulse_cnt += 1,
        }
    }

    fn run(&mut self) {
        self.pulses.push_back((
            String::from("button"),
            Pulse::Low,
            String::from("broadcaster"),
        ));

        while let Some((from_module_id, pulse, module_id)) = self.pulses.pop_front() {
            self.update_pulse_cnt(pulse);

            let module = self
                .modules
                .entry(module_id.clone())
                .or_insert(Module::new_dummy());

            if let Some(pulse) = module.run(
                from_module_id,
                pulse,
                self.sources.entry(module_id.clone()).or_default(),
            ) {
                for destination in self
                    .destinations
                    .entry(module_id.clone())
                    .or_default()
                    .iter()
                {
                    self.pulses
                        .push_back((module_id.clone(), pulse, destination.clone()));
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Device {
    let mut device = Device::new();

    let re_line = Regex::new(r"(.*) -> (.*)").unwrap();

    for line in input.lines() {
        let caps = re_line.captures(line).unwrap();

        let module = &caps[1];
        let destinations = caps[2]
            .split(',')
            .map(|x| x.trim().to_string())
            .collect::<Vec<_>>();

        let (id, module) = match module {
            "broadcaster" => ("broadcaster".to_string(), Module::new_broadcaster()),
            module if module.starts_with("%") => (module[1..].to_string(), Module::new_flip_flop()),
            module if module.starts_with("&") => {
                (module[1..].to_string(), Module::new_conjunction())
            }
            _ => unreachable!(),
        };

        device.register_module(id, module, destinations);
    }

    device
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut device = parse_input(input);

    for _ in 0..1000 {
        device.run();
    }

    Some(device.low_pulse_cnt * device.high_pulse_cnt)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
