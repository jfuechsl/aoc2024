use aoc2024::utils::file::load_file_lines;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl OpCode {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => panic!("Invalid OpCode"),
        }
    }

    fn get_operand_value(&self, computer: &Computer, opid: u8) -> u64 {
        match self {
            OpCode::Adv => Operand::new_combo(opid).value(computer),
            OpCode::Bxl => Operand::new_literal(opid).value(computer),
            OpCode::Bst => Operand::new_combo(opid).value(computer),
            OpCode::Jnz => Operand::new_literal(opid).value(computer),
            OpCode::Bxc => Operand::new_literal(opid).value(computer),
            OpCode::Out => Operand::new_combo(opid).value(computer),
            OpCode::Bdv => Operand::new_combo(opid).value(computer),
            OpCode::Cdv => Operand::new_combo(opid).value(computer),
        }
    }

    fn eval(&self, computer: &mut Computer, opid: u8) -> (bool, Option<u8>) {
        let operand_value = self.get_operand_value(computer, opid);
        match self {
            OpCode::Adv => {
                let num = computer.register_a;
                let denum = u64::pow(2, operand_value as u32);
                let result = num / denum;
                computer.register_a = result;
                (true, None)
            }
            OpCode::Bxl => {
                let lhs = computer.register_b;
                let rhs = operand_value;
                let result = lhs ^ rhs;
                computer.register_b = result;
                (true, None)
            }
            OpCode::Bst => {
                let result = operand_value % 8;
                computer.register_b = result;
                (true, None)
            }
            OpCode::Jnz => {
                if computer.register_a != 0 {
                    computer.instruction_pointer = operand_value as usize;
                    (false, None)
                } else {
                    (true, None)
                }
            }
            OpCode::Bxc => {
                let lhs = computer.register_b;
                let rhs = computer.register_c;
                let result = lhs ^ rhs;
                computer.register_b = result;
                (true, None)
            }
            OpCode::Out => (true, Some((operand_value % 8) as u8)),
            OpCode::Bdv => {
                let num = computer.register_a;
                let denum = u64::pow(2, operand_value as u32);
                let result = num / denum;
                computer.register_b = result;
                (true, None)
            }
            OpCode::Cdv => {
                let num = computer.register_a;
                let denum = u64::pow(2, operand_value as u32);
                let result = num / denum;
                computer.register_c = result;
                (true, None)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operand {
    Literal(u8),
    Combo(u8),
}

impl Operand {
    fn new_literal(value: u8) -> Self {
        Operand::Literal(value)
    }

    fn new_combo(value: u8) -> Self {
        Operand::Combo(value)
    }

    fn value(&self, computer: &Computer) -> u64 {
        match self {
            Operand::Literal(value) => *value as u64,
            Operand::Combo(value) => match value {
                0..=3 => *value as u64,
                4 => computer.register_a,
                5 => computer.register_b,
                6 => computer.register_c,
                7 => panic!("Combo operand 7 is invalid"),
                _ => panic!("Invalid combo operand"),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: usize,
    program: Vec<u8>,
}

impl Computer {
    fn from_file(lines: &Vec<String>) -> Self {
        let register_a = lines[0].split(": ").nth(1).unwrap().parse().unwrap();
        let register_b = lines[1].split(": ").nth(1).unwrap().parse().unwrap();
        let register_c = lines[2].split(": ").nth(1).unwrap().parse().unwrap();
        let program = lines[4]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        Computer {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
            program,
        }
    }

    fn run(&mut self) -> Vec<u8> {
        let mut output = Vec::new();
        while self.instruction_pointer < self.program.len() - 1 {
            let opcode = OpCode::from_u8(self.program[self.instruction_pointer]);
            let opid = self.program[self.instruction_pointer + 1];
            let (incr_ip, out) = opcode.eval(self, opid);
            if incr_ip {
                self.instruction_pointer += 2;
            }
            if let Some(out) = out {
                output.push(out);
            }
        }
        output
    }
}

fn reconstruct_program(computer: &Computer) -> u64 {
    /* Idea is to build up a stack of 3-bit blocks that make up a.
    Iterating backwards through the program we test which 3-bit block
    reproduces the program output to the end, then put that block on the stack.
    If no block works, retract the last block and try with a higher block value.
    */
    let pn = computer.program.len();
    let mut a_stack = Vec::new();
    let mut min_ai = 0;
    loop {
        let i = a_stack.len();
        let pi = pn - i - 1;
        let a = a_from_stack(&a_stack);
        let mut found_next = false;
        for ai in min_ai..8 {
            let a_test = (a << 3) | ai as u64;
            if check_a(a_test, pi, &computer) {
                a_stack.push(ai);
                found_next = true;
                break;
            }
        }
        if !found_next {
            assert!(!a_stack.is_empty());
            let last_ai = a_stack.pop().unwrap();
            assert!(last_ai < 7);
            min_ai = last_ai + 1;
        } else {
            min_ai = 0;
            if pi == 0 {
                break;
            }
        }
    }
    a_from_stack(&a_stack)
}

fn a_from_stack(a_stack: &Vec<u8>) -> u64 {
    let mut a = 0;
    for &val in a_stack.iter() {
        a = a << 3;
        a = a | val as u64;
    }
    a
}

fn check_a(a: u64, pi: usize, computer: &Computer) -> bool {
    let mut comp = computer.clone();
    comp.register_a = a;
    let output = comp.run();
    let psegment = &comp.program[pi..];
    if output.len() != psegment.len() {
        return false;
    }
    let output_matches = output.iter().zip(psegment.iter()).all(|(o, p)| *o == *p);
    output_matches
}

fn main() {
    let filename = "inputs/day17.txt";
    let lines = load_file_lines(filename).expect("Unable to load file");
    let computer = Computer::from_file(&lines);

    // Part 1
    let output = computer.clone().run();
    let output_str = output.iter().join(",");
    assert_eq!(output_str, "5,1,3,4,3,7,2,1,7");
    println!("Part 1: {}", output_str);

    // Part 2
    let a = reconstruct_program(&computer);
    assert_eq!(a, 216584205979245);
    println!("Part 2: {}", a);
    let matches = check_a(a, 0, &computer);
    assert!(matches);
}
