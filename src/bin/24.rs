use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut circuit, signals) = parse_input(input);

    circuit.run(signals);

    Some(circuit.find_z_wires_val())
}

pub fn part_two(input: &str) -> Option<String> {
    let (circuit, _) = parse_input(input);

    let mut faulty_wires = HashSet::new();
    let largest_z_wire_id = circuit.find_largest_z_wire_id();

    for gate in circuit.gates.iter() {
        let gate = gate.borrow_mut();

        let input_id1 = gate.input_id1.clone();
        let input_id2 = gate.input_id2.clone();
        let output_id = gate.output_id.clone();
        let output = gate.output.borrow_mut();

        if input_id1.starts_with("z") {
            faulty_wires.insert(input_id1.clone());
        }
        if input_id2.starts_with("z") {
            faulty_wires.insert(input_id2.clone());
        }

        if gate.logic != "XOR" && output_id.starts_with("z") && output_id != largest_z_wire_id {
            faulty_wires.insert(output_id.clone());
        }

        if gate.logic == "XOR"
            && !output_id.starts_with("z")
            && !((input_id1.starts_with("x") && input_id2.starts_with("y"))
                || (input_id1.starts_with("y") && input_id2.starts_with("x")))
        {
            faulty_wires.insert(output_id.clone());
        }

        if gate.logic == "XOR" && !output_id.starts_with("z") && output.output_gates.len() != 2 {
            faulty_wires.insert(output_id.clone());
        }

        if gate.logic == "AND"
            && !output_id.starts_with("z")
            && output.output_gates.len() != 1
            && !((input_id1 == "x00" && input_id2 == "y00")
                || (input_id1 == "y00" && input_id2 == "x00"))
        {
            faulty_wires.insert(output_id.clone());
        }
    }

    let mut faulty_wires = faulty_wires.into_iter().collect::<Vec<_>>();
    faulty_wires.sort();

    Some(faulty_wires.join(","))
}

#[derive(Debug)]
struct Circuit {
    wires: HashMap<String, Rc<RefCell<Wire>>>,
    gates: Vec<Rc<RefCell<Gate>>>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            wires: HashMap::new(),
            gates: vec![],
        }
    }

    fn add_gate(
        &mut self,
        input_wire1: String,
        input_wire2: String,
        logic: String,
        output_wire: String,
    ) {
        let output = self
            .wires
            .entry(output_wire.clone())
            .or_insert(Rc::new(RefCell::new(Wire::new())))
            .clone();

        let gate = Rc::new(RefCell::new(Gate::new(logic, output)));

        let inpu1 = self
            .wires
            .entry(input_wire1.clone())
            .or_insert(Rc::new(RefCell::new(Wire::new())))
            .clone();
        inpu1.borrow_mut().output_gates.push(gate.clone());

        let input2 = self
            .wires
            .entry(input_wire2.clone())
            .or_insert(Rc::new(RefCell::new(Wire::new())))
            .clone();
        input2.borrow_mut().output_gates.push(gate.clone());

        gate.borrow_mut().input_id1 = input_wire1;
        gate.borrow_mut().input_id2 = input_wire2;
        gate.borrow_mut().output_id = output_wire.to_string();
        self.gates.push(gate.clone());
    }

    fn run(&mut self, signals: Vec<(String, bool)>) {
        for (wire_id, signal) in signals {
            self.wires
                .get(&wire_id)
                .unwrap()
                .borrow_mut()
                .trigger(signal);
        }
    }

    fn find_z_wires_val(&self) -> u64 {
        let mut signals = vec![];

        for (wire_id, wire) in self.wires.iter() {
            if wire_id.starts_with('z') {
                signals.push((wire_id.to_string(), wire.borrow().signal.unwrap()));
            }
        }
        signals.sort();
        signals.reverse();

        let mut ans = 0;
        for (_, signal) in signals {
            ans <<= 1;

            if signal {
                ans += 1;
            }
        }

        ans
    }

    fn find_largest_z_wire_id(&self) -> &str {
        let mut ans = "z00";

        for (wire_id, _) in self.wires.iter() {
            if wire_id.starts_with('z') {
                ans = ans.max(wire_id);
            }
        }

        ans
    }
}

#[derive(Debug)]
struct Wire {
    signal: Option<bool>,
    output_gates: Vec<Rc<RefCell<Gate>>>,
}

impl Wire {
    fn new() -> Self {
        Wire {
            signal: None,
            output_gates: vec![],
        }
    }

    fn trigger(&mut self, signal: bool) {
        self.signal = Some(signal);

        for gate in self.output_gates.iter() {
            gate.borrow_mut().trigger(signal);
        }
    }
}

#[derive(Debug)]
struct Gate {
    signals: Vec<bool>,
    logic: String,
    output: Rc<RefCell<Wire>>,
    input_id1: String,
    input_id2: String,
    output_id: String,
}

impl Gate {
    fn new(logic: String, output: Rc<RefCell<Wire>>) -> Self {
        Gate {
            signals: vec![],
            logic,
            output,
            input_id1: "".to_string(),
            input_id2: "".to_string(),
            output_id: "".to_string(),
        }
    }

    fn trigger(&mut self, signal: bool) {
        self.signals.push(signal);
        if self.signals.len() < 2 {
            return;
        }

        let signal = match self.logic.as_str() {
            "AND" => self.signals[0] && self.signals[1],
            "OR" => self.signals[0] || self.signals[1],
            "XOR" => self.signals[0] != self.signals[1],
            _ => unreachable!(),
        };

        self.output.borrow_mut().trigger(signal);
    }
}

fn parse_input(input: &str) -> (Circuit, Vec<(String, bool)>) {
    let mut circuit = Circuit::new();
    let mut signals = vec![];

    let mut parsing_first_part = true;
    let re_gate_line = Regex::new(r"(.*) (.*) (.*) -> (.*)").unwrap();

    for line in input.lines() {
        if line.is_empty() {
            parsing_first_part = !parsing_first_part;
            continue;
        }

        if parsing_first_part {
            if let Some(line) = line.split_once(':') {
                let wire_id = line.0.to_string();
                let signal = match line.1.trim() {
                    "0" => false,
                    "1" => true,
                    _ => unreachable!(),
                };

                signals.push((wire_id, signal));
            }
        } else {
            let cap = re_gate_line.captures(line).unwrap();

            circuit.add_gate(
                cap[1].to_string(),
                cap[3].to_string(),
                cap[2].to_string(),
                cap[4].to_string(),
            );
        }
    }

    (circuit, signals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
