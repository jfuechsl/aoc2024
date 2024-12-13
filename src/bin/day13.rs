use core::f64;

use aoc2024::utils::file::load_file_lines;
use good_lp::{
    constraint, default_solver, variable, variables, ResolutionError, Solution, SolverModel,
};

#[derive(Debug)]
struct ClawMachine {
    prize: (u64, u64),
    a_dir: (u64, u64),
    b_dir: (u64, u64),
}

fn cost(a: u64, b: u64) -> u64 {
    3 * a + 1 * b
}

fn determinant(a: i64, b: i64, c: i64, d: i64) -> i64 {
    a * d - b * c
}

fn solve_ax_b(a: i64, b: i64, c: i64, d: i64, b1: i64, b2: i64) -> Option<(f64, f64)> {
    let det = determinant(a, b, c, d);
    if det == 0 {
        return None;
    }
    let x = determinant(b1, b, b2, d) as f64 / det as f64;
    let y = determinant(a, b1, c, b2) as f64 / det as f64;
    Some((x, y))
}

impl ClawMachine {
    fn new(
        prize_x: u64,
        prize_y: u64,
        a_dir_x: u64,
        a_dir_y: u64,
        b_dir_x: u64,
        b_dir_y: u64,
    ) -> Self {
        ClawMachine {
            prize: (prize_x, prize_y),
            a_dir: (a_dir_x, a_dir_y),
            b_dir: (b_dir_x, b_dir_y),
        }
    }

    fn solve_min_tokens(&self, max_presses_each: Option<u64>) -> Option<u64> {
        let (prize_x, prize_y) = self.prize;
        let (a_dir_x, a_dir_y) = self.a_dir;
        let (b_dir_x, b_dir_y) = self.b_dir;
        let (prize_x, prize_y) = (prize_x as f64, prize_y as f64);
        let (a_dir_x, a_dir_y) = (a_dir_x as f64, a_dir_y as f64);
        let (b_dir_x, b_dir_y) = (b_dir_x as f64, b_dir_y as f64);

        let mut vars = variables!();
        let (a, b) = if let Some(max_presses_each) = max_presses_each {
            (
                vars.add(
                    variable()
                        .integer()
                        .min(0.0)
                        .max(max_presses_each as f64)
                        .name("a"),
                ),
                vars.add(
                    variable()
                        .integer()
                        .min(0.0)
                        .max(max_presses_each as f64)
                        .name("b"),
                ),
            )
        } else {
            (
                vars.add(variable().integer().min(0.0).name("a")),
                vars.add(variable().integer().min(0.0).name("b")),
            )
        };
        let mut model = vars
            .minimise(3.0 * a + 1.0 * b)
            .using(default_solver)
            .with(constraint!(a * a_dir_x + b * b_dir_x == prize_x))
            .with(constraint!(a * a_dir_y + b * b_dir_y == prize_y));
        model.set_parameter("log", "0");
        let solution = model.solve();
        if solution.is_err() {
            match solution.err().unwrap() {
                ResolutionError::Infeasible => (),
                e @ _ => println!("Error: {:?}", e),
            }
            // NB: Can't trust that the solution is really infeasible.
            // So we need to check the solution manually.
            if let Some((alt_a, alt_b)) = solve_ax_b(
                a_dir_x as i64,
                b_dir_x as i64,
                a_dir_y as i64,
                b_dir_y as i64,
                prize_x as i64,
                prize_y as i64,
            ) {
                let a_val = alt_a.round() as u64;
                let b_val = alt_b.round() as u64;
                let cost_value = cost(a_val, b_val);
                let x_pos = a_val * a_dir_x as u64 + b_val * b_dir_x as u64;
                let y_pos = a_val * a_dir_y as u64 + b_val * b_dir_y as u64;
                if x_pos == prize_x as u64 && y_pos == prize_y as u64 {
                    return Some(cost_value);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            let solution = solution.unwrap();
            let a_val = solution.value(a);
            let b_val = solution.value(b);
            let a_val = a_val.round() as u64;
            let b_val = b_val.round() as u64;
            let cost_value = cost(a_val, b_val);
            let x_pos = a_val * a_dir_x as u64 + b_val * b_dir_x as u64;
            let y_pos = a_val * a_dir_y as u64 + b_val * b_dir_y as u64;
            assert_eq!(x_pos, prize_x as u64);
            assert_eq!(y_pos, prize_y as u64);
            return Some(cost_value);
        }
    }
}

fn parse_input(input_lines: &Vec<String>, prize_offset: u64) -> Vec<ClawMachine> {
    input_lines
        .chunk_by(|_, l| !l.is_empty())
        .map(|chunk| {
            let callibrations: Vec<_> = chunk.into_iter().filter(|l| !l.is_empty()).collect();
            assert_eq!(callibrations.len(), 3);
            let button_a_str = callibrations[0];
            let button_b_str = callibrations[1];
            let prize_str = callibrations[2];
            let button_a_regex = regex::Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
            let button_a_caps = button_a_regex.captures(button_a_str).unwrap();
            let button_a_x = button_a_caps
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            let button_a_y = button_a_caps
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            let button_b_regex = regex::Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
            let button_b_caps = button_b_regex.captures(button_b_str).unwrap();
            let button_b_x = button_b_caps
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            let button_b_y = button_b_caps
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            let prize_regex = regex::Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
            let prize_caps = prize_regex.captures(prize_str).unwrap();
            let prize_x =
                prize_caps.get(1).unwrap().as_str().parse::<u64>().unwrap() + prize_offset;
            let prize_y =
                prize_caps.get(2).unwrap().as_str().parse::<u64>().unwrap() + prize_offset;
            ClawMachine::new(
                prize_x, prize_y, button_a_x, button_a_y, button_b_x, button_b_y,
            )
        })
        .collect()
}

fn main() {
    let filename = "inputs/day13.txt";
    let input_lines = load_file_lines(filename).expect("Failed to read file");

    // Part 1
    let claw_machines = parse_input(&input_lines, 0);
    let mut total_cost = 0;
    let mut num_infeasible = 0;
    for claw_machine in claw_machines {
        if let Some(cost) = claw_machine.solve_min_tokens(Some(100)) {
            total_cost += cost;
        } else {
            num_infeasible += 1;
        }
    }
    assert_eq!(total_cost, 30413);
    println!("Total cost: {} ({} infeasible)", total_cost, num_infeasible);

    // Part 2
    let claw_machines = parse_input(&input_lines, 10000000000000);
    let mut total_cost = 0;
    let mut num_infeasible = 0;
    for claw_machine in claw_machines {
        if let Some(cost) = claw_machine.solve_min_tokens(None) {
            total_cost += cost;
        } else {
            num_infeasible += 1;
        }
    }
    assert_eq!(total_cost, 92827349540204);
    println!("Total cost: {} ({} infeasible)", total_cost, num_infeasible);
}
