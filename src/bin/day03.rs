use aoc2024::utils::file::read_lines;
use regex::Regex;

enum Instr {
    Mul(i32, i32),
    Do,
    Dont,
}

fn parse_instruction(cap: regex::Captures) -> Instr {
    if cap.get(1).is_some() {
        let first = cap[2].parse::<i32>().expect("Failed to parse first number");
        let second = cap[3]
            .parse::<i32>()
            .expect("Failed to parse second number");
        Instr::Mul(first, second)
    } else if cap.get(4).is_some() {
        Instr::Do
    } else if cap.get(5).is_some() {
        Instr::Dont
    } else {
        panic!("Invalid instruction")
    }
}

fn extract_instructions(text: &str) -> Vec<Instr> {
    let re = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)")
        .expect("Invalid regex pattern");
    re.captures_iter(text)
        .map(|cap| parse_instruction(cap))
        .collect()
}

fn exec_all_mul_instructions(instructions: &Vec<Instr>) -> Vec<i32> {
    instructions
        .iter()
        .filter_map(|instr| match instr {
            Instr::Mul(first, second) => Some((first, second)),
            _ => None,
        })
        .map(|(first, second)| first * second)
        .collect()
}

fn exec_enabled_mul_instructions(instructions: &Vec<Instr>) -> Vec<i32> {
    let mut enabled = true;
    let mut results = Vec::new();
    for instr in instructions {
        match instr {
            Instr::Do => enabled = true,
            Instr::Dont => enabled = false,
            Instr::Mul(first, second) => {
                if enabled {
                    results.push(first * second);
                }
            }
        }
    }
    results
}

fn sum_items(items: Vec<i32>) -> i32 {
    items.iter().sum()
}

fn main() {
    let program = read_lines("inputs/day03.txt").expect("Could not read file");
    let instructions = extract_instructions(program.as_str());

    // Part 1
    let multiplications = exec_all_mul_instructions(&instructions);
    let sum = sum_items(multiplications);
    println!("Sum of all multiplications: {}", sum);

    // Part 2
    let multiplications = exec_enabled_mul_instructions(&instructions);
    let sum = sum_items(multiplications);
    println!("Sum of enabled multiplications: {}", sum);
}
