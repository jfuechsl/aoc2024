use std::collections::{HashMap, HashSet};

use aoc2024::utils::file::load_file_lines;

fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

fn prune(number: usize) -> usize {
    number % 16777216
}

fn next_secret_number(number: usize) -> usize {
    let mut result = prune(mix(number, number * 64));
    result = prune(mix(result, result / 32));
    prune(mix(result, result * 2048))
}

fn iterated_secret_numbers(start: usize, n: usize) -> usize {
    let mut num = start;
    for _ in 0..n {
        num = next_secret_number(num);
    }
    num
}

fn price(rand_num: usize) -> i64 {
    (rand_num % 10) as i64
}

fn price_sequence(start: usize, n: usize) -> Vec<i64> {
    let mut num = start;
    let mut result = Vec::new();
    for _ in 0..n {
        let next_num = next_secret_number(num);
        result.push(price(next_num));
        num = next_num;
    }
    result
}

fn diff_sequence(start: usize, n: usize) -> Vec<i64> {
    let mut num = start;
    let mut result = Vec::new();
    let mut p = price(num);
    for _ in 0..n {
        let next_num = next_secret_number(num);
        let np = price(next_num);
        let diff = np - p;
        result.push(diff);
        num = next_num;
        p = np;
    }
    result
}

fn main() {
    let filename = "inputs/day22.txt";
    let secret_numbers: Vec<usize> = load_file_lines(filename)
        .expect("Invalid filename")
        .iter()
        .map(|s| s.parse::<usize>().expect("Invalid number"))
        .collect::<Vec<usize>>();

    // Part 1
    let sum_numbers: usize = secret_numbers
        .iter()
        .map(|&start| iterated_secret_numbers(start, 2000))
        .sum();
    // assert_eq!(sum_numbers, 13584398738);
    println!("Sum of secret numbers: {}", sum_numbers);

    // Part 2
    let diffs: Vec<Vec<i64>> = secret_numbers
        .iter()
        .map(|&start| diff_sequence(start, 2000))
        .collect();
    let prices: Vec<Vec<i64>> = secret_numbers
        .iter()
        .map(|&start| price_sequence(start, 2000))
        .collect();
    let mut patterns = HashSet::new();
    for (ps, ds) in prices.iter().zip(diffs.iter()) {
        assert_eq!(ps.len(), ds.len());
        for i in 3..ps.len() {
            let pattern = &ds[i - 3..=i];
            patterns.insert(pattern);
        }
    }
    let mut pattern_payoffs = HashMap::new();
    for pattern in patterns.into_iter() {
        let mut payoff = 0;
        for (ps, ds) in prices.iter().zip(diffs.iter()) {
            for i in 3..ps.len() {
                let current_pattern = &ds[i - 3..=i];
                if current_pattern == pattern {
                    payoff += ps[i];
                    break;
                }
            }
        }
        pattern_payoffs.insert(pattern, payoff);
    }
    let max_payoff = pattern_payoffs.values().max().unwrap();
    assert_eq!(*max_payoff, 1612);
    println!("Max payoff: {}", max_payoff);
}
