use std::collections::{HashMap, HashSet};

use aoc2024::utils::file::load_file_lines;

fn pattern_len_from_stack(stack: &Vec<(String, usize)>) -> usize {
    stack.iter().map(|(s, _)| s.len()).sum()
}

fn reconstruct_pattern(
    pattern: &String,
    available_patterns: &HashSet<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    let mut pattern_stack = Vec::new();
    let mut num_combinations = 0;
    let mut min_next_len = 1;
    loop {
        let start_i = pattern_len_from_stack(&pattern_stack);
        let mut found_next = false;
        // Check cache
        let rest_of_pattern = pattern[start_i..].to_string();
        if rest_of_pattern.len() >= min_next_len && cache.contains_key(&rest_of_pattern) {
            let cached_count = cache.get(&rest_of_pattern).unwrap();
            found_next = true;
            pattern_stack.push((rest_of_pattern, *cached_count));
        } else {
            // Continue if no cache hit
            let mut next_len = min_next_len;
            while start_i + next_len <= pattern.len() {
                let next_pattern = &pattern[start_i..start_i + next_len];
                if available_patterns.contains(next_pattern) {
                    pattern_stack.push((next_pattern.to_string(), 1));
                    found_next = true;
                    break;
                }
                next_len += 1;
            }
        }
        if !found_next {
            if pattern_stack.is_empty() {
                break;
            }
            let (last_pattern, _) = pattern_stack.pop().unwrap();
            min_next_len = last_pattern.len() + 1;
        } else {
            if pattern_len_from_stack(&pattern_stack) == pattern.len() {
                let (last_pattern, new_combinations) = pattern_stack.pop().unwrap();
                num_combinations += new_combinations;
                min_next_len = last_pattern.len() + 1;
            } else {
                min_next_len = 1;
            }
        }
    }
    if num_combinations > 0 {
        cache.insert(pattern.clone(), num_combinations);
    }
    num_combinations
}

fn cached_reconstruct_pattern(
    pattern: &String,
    available_patterns: &HashSet<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    let n = pattern.len();
    for pa in (0..n).rev() {
        let pattern_remaining = &pattern[pa..];
        reconstruct_pattern(&pattern_remaining.to_string(), available_patterns, cache);
    }
    reconstruct_pattern(pattern, available_patterns, cache)
}

fn main() {
    let filename = "inputs/day19.txt";
    let mut lines = load_file_lines(filename).expect("Invalid filename");

    // Extract the first line and split by commas
    let first_line = lines.remove(0);
    let available_pattern_strings: HashSet<String> =
        first_line.split(", ").map(|s| s.to_string()).collect();

    // Collect the rest of the file lines after the blank line
    let patterns: Vec<String> = lines
        .into_iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .collect();

    // Part 1 & Part 2
    let mut num_possible = 0;
    let mut sum_combinations = 0;
    let mut cache = HashMap::new();
    for pattern in patterns {
        let num_combinations =
            cached_reconstruct_pattern(&pattern, &available_pattern_strings, &mut cache);
        if num_combinations > 0 {
            num_possible += 1;
        }
        sum_combinations += num_combinations;
    }
    println!("Part 1: {}", num_possible);
    assert_eq!(num_possible, 267);
    println!("Part 2: {}", sum_combinations);
    assert_eq!(sum_combinations, 796449099271652);
}
