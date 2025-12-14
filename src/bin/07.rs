use std::collections::HashMap;

advent_of_code::solution!(7);

type WireId<'a> = &'a str;

#[derive(Clone, Copy, Debug)]
enum Input<'a> {
    Val(u16),
    Wire(WireId<'a>),
}

impl<'a> Input<'a> {
    fn parse(s: &'a str) -> Self {
        s.parse::<u16>()
            .map(Input::Val)
            .unwrap_or_else(|_| Input::Wire(s))
    }

    fn resolve(
        &self,
        wires: &HashMap<WireId<'a>, Gate<'a>>,
        cache: &mut HashMap<WireId<'a>, u16>,
    ) -> u16 {
        match self {
            Input::Val(v) => *v,
            Input::Wire(id) => solve(id, wires, cache),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Gate<'a> {
    Direct(Input<'a>),
    And(Input<'a>, Input<'a>),
    Or(Input<'a>, Input<'a>),
    LShift(Input<'a>, u16),
    RShift(Input<'a>, u16),
    Not(Input<'a>),
}

pub fn part_one(input: &str) -> Option<u16> {
    let gates = parse_circuit(input);
    let mut cache = HashMap::new();

    Some(solve("a", &gates, &mut cache))
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut gates = parse_circuit(input);

    let val_a = {
        let mut cache = HashMap::new();
        solve("a", &gates, &mut cache)
    };

    gates.insert("b", Gate::Direct(Input::Val(val_a)));
    let mut cache = HashMap::new();

    Some(solve("a", &gates, &mut cache))
}

fn parse_circuit(input: &str) -> HashMap<&str, Gate<'_>> {
    input
        .lines()
        .map(|line| {
            let (expr, target) = line.split_once(" -> ").expect("Malformed line");
            let parts: Vec<&str> = expr.split_ascii_whitespace().collect();

            let gate = match parts.as_slice() {
                [src] => Gate::Direct(Input::parse(src)),
                ["NOT", src] => Gate::Not(Input::parse(src)),
                [lhs, "AND", rhs] => Gate::And(Input::parse(lhs), Input::parse(rhs)),
                [lhs, "OR", rhs] => Gate::Or(Input::parse(lhs), Input::parse(rhs)),
                [lhs, "LSHIFT", amt] => Gate::LShift(Input::parse(lhs), amt.parse().unwrap()),
                [lhs, "RSHIFT", amt] => Gate::RShift(Input::parse(lhs), amt.parse().unwrap()),
                _ => panic!("Unknown gate: {}", expr),
            };
            (target, gate)
        })
        .collect()
}

fn solve<'a>(
    wire: WireId<'a>,
    gates: &HashMap<WireId<'a>, Gate<'a>>,
    cache: &mut HashMap<WireId<'a>, u16>,
) -> u16 {
    if let Some(&val) = cache.get(wire) {
        return val;
    }

    let gate = gates.get(wire).expect("Wire not connected");

    let result = match gate {
        Gate::Direct(inp) => inp.resolve(gates, cache),
        Gate::And(lhs, rhs) => lhs.resolve(gates, cache) & rhs.resolve(gates, cache),
        Gate::Or(lhs, rhs) => lhs.resolve(gates, cache) | rhs.resolve(gates, cache),
        Gate::LShift(lhs, amt) => lhs.resolve(gates, cache) << amt,
        Gate::RShift(lhs, amt) => lhs.resolve(gates, cache) >> amt,
        Gate::Not(inp) => !inp.resolve(gates, cache),
    };

    cache.insert(wire, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
