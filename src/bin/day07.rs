use aoc2024::utils::file::load_file_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

impl Operation {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concat => format!("{}{}", a, b).parse().unwrap(),
        }
    }
}

struct OperationPermutations {
    n: usize,
    include_concat: bool,
    current: Vec<Operation>,
    done: bool,
}

impl OperationPermutations {
    fn new(n: usize, include_concat: bool) -> Self {
        OperationPermutations {
            n,
            include_concat,
            current: vec![Operation::Add; n],
            done: false,
        }
    }
}

impl Iterator for OperationPermutations {
    type Item = Vec<Operation>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.current.clone();

        for i in (0..self.n).rev() {
            self.current[i] = match self.current[i] {
                Operation::Add => Operation::Multiply,
                Operation::Multiply => {
                    if self.include_concat {
                        Operation::Concat
                    } else {
                        if i == 0 {
                            self.done = true;
                        }
                        Operation::Add
                    }
                }
                Operation::Concat => {
                    if i == 0 {
                        self.done = true;
                    }
                    Operation::Add
                }
            };

            if self.current[i] != Operation::Add {
                break;
            }
        }

        Some(result)
    }
}

#[derive(Debug, Clone)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split(": ");
        let result = parts.next().unwrap().parse().unwrap();
        let numbers = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|n| n.parse().unwrap())
            .collect();
        Equation { result, numbers }
    }

    fn is_correct(&self, ops: &[Operation]) -> bool {
        assert!(ops.len() + 1 == self.numbers.len());
        let mut result = self.numbers[0];
        for (op, &number) in ops.iter().zip(self.numbers.iter().skip(1)) {
            result = op.apply(result, number);
        }
        result == self.result
    }

    fn can_be_correct(&self, include_concat: bool) -> bool {
        for ops in OperationPermutations::new(self.numbers.len() - 1, include_concat) {
            if self.is_correct(&ops) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let lines = load_file_lines("inputs/day07.txt").expect("Failed to load file");
    let equations: Vec<Equation> = lines.iter().map(|line| Equation::from_line(line)).collect();

    // Part 1
    let sum_correct: i64 = equations
        .iter()
        .filter(|eq| eq.can_be_correct(false))
        .map(|eq| eq.result)
        .sum();
    println!("Sum of correct results (w/o concat): {}", sum_correct);

    // Part 2
    let sum_correct: i64 = equations
        .iter()
        .filter(|eq| eq.can_be_correct(true))
        .map(|eq| eq.result)
        .sum();
    println!("Sum of correct results (w/ concat): {}", sum_correct);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_permutations_no_concat() {
        let mut permutations = OperationPermutations::new(2, false);
        assert_eq!(
            permutations.next(),
            Some(vec![Operation::Add, Operation::Add])
        );
        assert_eq!(
            permutations.next(),
            Some(vec![Operation::Add, Operation::Multiply])
        );
        assert_eq!(
            permutations.next(),
            Some(vec![Operation::Multiply, Operation::Add])
        );
        assert_eq!(
            permutations.next(),
            Some(vec![Operation::Multiply, Operation::Multiply])
        );
        assert_eq!(permutations.next(), None);
    }

    #[test]
    fn test_operation_permutations_with_concat() {
        let mut permutations = OperationPermutations::new(2, true);
        assert_eq!(
            permutations.next(),
            Some(vec![Operation::Add, Operation::Add])
        );
        assert_eq!(
            permutations.next(),
            Some(vec![Operation::Add, Operation::Multiply])
        );
        assert_eq!(
            permutations.next(),
            Some(vec![Operation::Add, Operation::Concat])
        );
        assert_eq!(
            permutations.next(),
            Some(vec![Operation::Multiply, Operation::Add])
        );
        assert_eq!(
            permutations.next(),
            Some(vec![Operation::Multiply, Operation::Multiply])
        );
        assert_eq!(
            permutations.next(),
            Some(vec![Operation::Multiply, Operation::Concat])
        );
        assert_eq!(
            permutations.next(),
            Some(vec![Operation::Concat, Operation::Add])
        );
        assert_eq!(
            permutations.next(),
            Some(vec![Operation::Concat, Operation::Multiply])
        );
        assert_eq!(
            permutations.next(),
            Some(vec![Operation::Concat, Operation::Concat])
        );
        assert_eq!(permutations.next(), None);
    }

    #[test]
    fn test_equation_is_correct() {
        let equation = Equation {
            result: 6,
            numbers: vec![1, 2, 3],
        };
        assert!(equation.is_correct(&[Operation::Add, Operation::Add]));
        assert!(!equation.is_correct(&[Operation::Multiply, Operation::Add]));
    }

    #[test]
    fn test_equation_can_be_correct() {
        let equation = Equation {
            result: 6,
            numbers: vec![1, 2, 3],
        };
        assert!(equation.can_be_correct(false));

        let equation = Equation {
            result: 10,
            numbers: vec![1, 2, 3],
        };
        assert!(!equation.can_be_correct(false));
    }
}
