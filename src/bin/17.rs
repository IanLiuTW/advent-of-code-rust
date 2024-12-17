use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let mut program = parse_input(input);
    let output = program.run();
    let output = output.iter().map(|x| x.to_string()).join(",");

    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut program = parse_input(input);
    let target = program.program.clone();

    let mut reg_a = 1; // starts from 0 won't work since the multiplication of 8
    'outer: loop {
        program.reg_a = reg_a;
        program.reg_b = 0;
        program.reg_c = 0;

        let output = program.run();

        for (o, t) in output.iter().rev().zip(target.iter().rev()) {
            if o != t {
                reg_a += 1;
                continue 'outer;
            }
        }
        if output.len() < target.len() {
            reg_a *= 8;
        } else {
            break;
        }
    }

    Some(reg_a)
}

#[derive(Debug, Clone)]
struct Program {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    program: Vec<u64>,
    ins_ptr: usize,
}

impl Program {
    fn new(reg_a: u64, reg_b: u64, reg_c: u64, program: Vec<u64>) -> Self {
        Program {
            reg_a,
            reg_b,
            reg_c,
            program,
            ins_ptr: 0,
        }
    }

    fn run(&mut self) -> Vec<u64> {
        let mut output: Vec<u64> = vec![];

        self.ins_ptr = 0;
        while self.ins_ptr < self.program.len() {
            let opcode = OpCode(self.program[self.ins_ptr]);
            let operand = Operand(self.program[self.ins_ptr + 1]);

            match opcode.eval(operand, self) {
                OpCodeResult::None => {
                    self.ins_ptr += 2;
                }
                OpCodeResult::Output(val) => {
                    output.push(val);
                    self.ins_ptr += 2;
                }
                OpCodeResult::Jump(i) => {
                    self.ins_ptr = i;
                }
            }
        }

        output
    }
}

#[derive(Debug, Clone, Copy)]
struct OpCode(u64);

impl OpCode {
    fn eval(&self, operand: Operand, program: &mut Program) -> OpCodeResult {
        match self.0 {
            0 => OpCode::adv(operand, program),
            1 => OpCode::bxl(operand, program),
            2 => OpCode::bst(operand, program),
            3 => OpCode::jnz(operand, program),
            4 => OpCode::bxc(program),
            5 => OpCode::out(operand, program),
            6 => OpCode::bdv(operand, program),
            7 => OpCode::cdv(operand, program),
            _ => unreachable!(),
        }
    }

    fn adv(operand: Operand, program: &mut Program) -> OpCodeResult {
        program.reg_a /= 2_u64.pow(operand.get_combo(program) as u32);
        OpCodeResult::None
    }

    fn bxl(operand: Operand, program: &mut Program) -> OpCodeResult {
        program.reg_b ^= operand.get_literal();
        OpCodeResult::None
    }

    fn bst(operand: Operand, program: &mut Program) -> OpCodeResult {
        program.reg_b = operand.get_combo(program) % 8;
        OpCodeResult::None
    }

    fn jnz(operand: Operand, program: &mut Program) -> OpCodeResult {
        if program.reg_a == 0 {
            OpCodeResult::None
        } else {
            OpCodeResult::Jump(operand.get_literal() as usize)
        }
    }

    fn bxc(program: &mut Program) -> OpCodeResult {
        program.reg_b ^= program.reg_c;
        OpCodeResult::None
    }

    fn out(operand: Operand, program: &mut Program) -> OpCodeResult {
        OpCodeResult::Output(operand.get_combo(program) % 8)
    }

    fn bdv(operand: Operand, program: &mut Program) -> OpCodeResult {
        program.reg_b = program.reg_a / 2_u64.pow(operand.get_combo(program) as u32);
        OpCodeResult::None
    }

    fn cdv(operand: Operand, program: &mut Program) -> OpCodeResult {
        program.reg_c = program.reg_a / 2_u64.pow(operand.get_combo(program) as u32);
        OpCodeResult::None
    }
}

enum OpCodeResult {
    None,
    Output(u64),
    Jump(usize),
}

#[derive(Debug, Clone, Copy)]
struct Operand(u64);

impl Operand {
    fn get_literal(self) -> u64 {
        self.0
    }

    fn get_combo(self, program: &Program) -> u64 {
        match self.0 {
            0..=3 => self.0,
            4 => program.reg_a,
            5 => program.reg_b,
            6 => program.reg_c,
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> Program {
    let mut input = input.lines();

    let reg_re = Regex::new(r"Register .: (.*)").unwrap();
    let reg_a = reg_re.captures(input.next().unwrap()).unwrap()[1]
        .parse()
        .unwrap();
    let reg_b = reg_re.captures(input.next().unwrap()).unwrap()[1]
        .parse()
        .unwrap();
    let reg_c = reg_re.captures(input.next().unwrap()).unwrap()[1]
        .parse()
        .unwrap();

    let input = input
        .nth(1)
        .unwrap()
        .trim_start_matches("Program: ")
        .split(',');

    let mut program = vec![];
    for c in input {
        program.push(c.parse().unwrap());
    }

    Program::new(reg_a, reg_b, reg_c, program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let case = "Register A: 729\nRegister B: 0\nRegister C: 0\n\nProgram: 0,1,5,4,3,0";
        let result = part_one(case);
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
