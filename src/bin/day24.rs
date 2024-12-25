use core::panic;
use std::{
    collections::{BTreeSet, HashMap},
    fmt::Display,
};

use aoc2024::utils::file::load_file_lines;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operator {
    AND,
    OR,
    XOR,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            Self::AND => "AND",
            Self::OR => "OR",
            Self::XOR => "XOR",
        };
        write!(f, "{}", op_str)
    }
}

impl Operator {
    fn from_str(op: &str) -> Self {
        match op {
            "AND" => Self::AND,
            "OR" => Self::OR,
            "XOR" => Self::XOR,
            _ => panic!("Invalid operator"),
        }
    }

    fn eval(&self, operand1: u8, operand2: u8) -> u8 {
        match self {
            Self::AND => operand1 & operand2,
            Self::OR => operand1 | operand2,
            Self::XOR => operand1 ^ operand2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Operation {
    operator: Operator,
    operand1: String,
    operand2: String,
    result: String,
}

impl Operation {
    fn from_str(line: String) -> Self {
        let parts: Vec<&str> = line.split(" -> ").collect();
        assert_eq!(parts.len(), 2);
        let left_parts: Vec<&str> = parts[0].split_whitespace().collect();
        assert_eq!(left_parts.len(), 3);
        let operator = Operator::from_str(left_parts[1]);
        let mut operands = [left_parts[0], left_parts[2]];
        operands.sort();
        let operand1 = operands[0].to_string();
        let operand2 = operands[1].to_string();
        let result = parts[1].to_string();
        Self {
            operator,
            operand1,
            operand2,
            result,
        }
    }

    fn eval(&self, operand1_value: u8, operand2_value: u8) -> u8 {
        self.operator.eval(operand1_value, operand2_value)
    }

    fn is_input_op(&self) -> bool {
        self.operand1.starts_with("x") && self.operand2.starts_with("y")
    }

    fn input_offset(&self) -> Option<usize> {
        if self.is_input_op() {
            let op1 = parse_var(&self.operand1);
            let op2 = parse_var(&self.operand2);
            assert_eq!(op1.1, op2.1);
            let offset = op1.1;
            Some(offset)
        } else {
            None
        }
    }

    fn is_output_op(&self) -> bool {
        self.result.starts_with("z")
    }

    fn output_offset(&self) -> Option<usize> {
        if self.is_output_op() {
            let op = parse_var(&self.result);
            let offset = op.1;
            Some(offset)
        } else {
            None
        }
    }

    fn is_xor(&self) -> bool {
        self.operator == Operator::XOR
    }
}

#[derive(Debug, Clone)]
struct Circuit {
    operations: HashMap<String, Operation>,
    init_map: HashMap<String, u8>,
    result_vars: Vec<(String, usize)>,
}

fn parse_var(var: &str) -> (char, usize) {
    let var_id = var.chars().next().expect("Invalid var");
    let var_pos = var[1..].parse().expect("Invalid var");
    (var_id, var_pos)
}

impl Circuit {
    fn from_input(init_values: Vec<(String, u8)>, operations: Vec<String>) -> Self {
        let mut operations_map = HashMap::new();
        let mut init_map = HashMap::new();
        let mut result_vars = Vec::new();
        for (var, value) in init_values {
            init_map.insert(var, value);
        }
        for operation in operations {
            let op = Operation::from_str(operation);
            if op.is_output_op() {
                let offset = op.output_offset().expect("Invalid input offset");
                result_vars.push((op.result.clone(), offset));
            }
            operations_map.insert(op.result.clone(), op);
        }
        Self {
            operations: operations_map,
            init_map,
            result_vars,
        }
    }

    fn eval_var(&self, var: &String) -> u8 {
        if let Some(value) = self.init_map.get(var) {
            return *value;
        }
        let operation = self.operations.get(var).expect("Invalid var");
        let operand1_value = self.eval_var(&operation.operand1);
        let operand2_value = self.eval_var(&operation.operand2);
        let result_value = operation.eval(operand1_value, operand2_value);
        result_value
    }

    fn eval(&self) -> u64 {
        let mut result: u64 = 0;
        self.result_vars.iter().for_each(|(var, offset)| {
            let value = self.eval_var(var);
            result |= (value as u64) << offset;
        });
        result
    }

    fn max_input_offset(&self) -> usize {
        self.init_map
            .keys()
            .map(|var| parse_var(var.as_str()).1)
            .max()
            .unwrap()
    }

    fn find_xor_with_input_xor(&self, n: usize) -> Option<String> {
        for (op_name, op) in self.operations.iter() {
            if op.is_xor() {
                let arg1_op = self.operations.get(&op.operand1);
                let arg2_op = self.operations.get(&op.operand2);
                if let (Some(arg1_op), Some(arg2_op)) = (arg1_op, arg2_op) {
                    let ops = [arg1_op, arg2_op];
                    for iop in ops {
                        if iop.is_xor() && iop.is_input_op() {
                            let offset = iop.input_offset().expect("Invalid input offset");
                            if offset == n {
                                return Some(op_name.clone());
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn find_results_to_swap(&self) -> BTreeSet<String> {
        let max_input_offset = self.max_input_offset();
        let mut swaps = BTreeSet::new();
        for (res_var, offset) in self.result_vars.iter() {
            let res_op = self.operations.get(res_var).expect("Invalid result var");
            if !res_op.is_xor() {
                if *offset <= max_input_offset {
                    let swp = self.find_xor_with_input_xor(*offset);
                    if let Some(swp) = swp {
                        swaps.insert(swp);
                        swaps.insert(res_var.clone());
                    }
                }
            }
        }
        swaps
    }

    fn find_input_xor_var(&self, n: usize) -> Option<String> {
        for (op_name, op) in self.operations.iter() {
            if op.is_xor() && op.is_input_op() && op.input_offset().unwrap() == n {
                return Some(op_name.clone());
            }
        }
        None
    }

    fn find_result_input_xor_to_swap(&self) -> BTreeSet<String> {
        let max_input_offset = self.max_input_offset();
        let mut swaps = BTreeSet::new();
        for (res_var, offset) in self.result_vars.iter() {
            if *offset == 0 || *offset > max_input_offset {
                continue;
            }
            let res_op = self.operations.get(res_var).expect("Invalid result var");
            if res_op.is_xor() {
                let op1 = self
                    .operations
                    .get(&res_op.operand1)
                    .expect("Invalid operand");
                let op2 = self
                    .operations
                    .get(&res_op.operand2)
                    .expect("Invalid operand");
                for op in [op1, op2] {
                    if op.is_input_op() && op.input_offset().unwrap() == *offset && !op.is_xor() {
                        let xor_op_var = self.find_input_xor_var(*offset).unwrap();
                        swaps.insert(op.result.clone());
                        swaps.insert(xor_op_var);
                    }
                }
            }
        }
        swaps
    }
}

fn main() {
    let filename = "inputs/day24.txt";
    let lines = load_file_lines(filename).expect("File not found");

    let init_values: Vec<_> = lines
        .iter()
        .take_while(|&line| !line.is_empty())
        .cloned()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            assert_eq!(parts.len(), 2);
            let value: u8 = parts[1].parse().expect("Invalid value");
            (parts[0].to_string(), value)
        })
        .collect();

    let operations: Vec<_> = lines
        .iter()
        .skip_while(|&line| !line.is_empty())
        .skip(1)
        .cloned()
        .collect();

    let circuit = Circuit::from_input(init_values, operations);

    // Part 1
    let result = circuit.eval();
    assert_eq!(result, 51657025112326);
    println!("Part 1 result: {}", result);

    // Part 2
    let swaps1 = circuit.find_results_to_swap();
    let swaps2 = circuit.find_result_input_xor_to_swap();
    let swaps: BTreeSet<String> = swaps1.union(&swaps2).cloned().collect();
    assert_eq!(swaps.len(), 8);
    let swaps_str = swaps.iter().join(",");
    assert_eq!(swaps_str, "gbf,hdt,jgt,mht,nbf,z05,z09,z30");
    println!("Swaps (Part 2): {}", swaps_str);
}
