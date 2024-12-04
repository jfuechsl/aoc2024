use std::collections::HashMap;

use aoc2024::utils::file::load_file_lines;

fn parse_two_integers(input: &str) -> Option<(i32, i32)> {
    let mut parts = input.split_whitespace();
    if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
        if let (Ok(first_num), Ok(second_num)) = (first.parse::<i32>(), second.parse::<i32>()) {
            return Some((first_num, second_num));
        }
    }
    None
}

fn extract_numbers(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();
    load_file_lines(filename)
        .expect("Could not load file")
        .iter()
        .map(|line| parse_two_integers(line).expect("Could not parse line"))
        .for_each(|(first, second)| {
            left_numbers.push(first);
            right_numbers.push(second);
        });
    left_numbers.sort();
    right_numbers.sort();
    (left_numbers, right_numbers)
}

fn compute_total_distance(left_numbers: &Vec<i32>, right_numbers: &Vec<i32>) -> i32 {
    let mut total_dist = 0;
    for (left, right) in left_numbers.iter().zip(right_numbers.iter()) {
        total_dist += (right - left).abs();
    }
    total_dist
}

fn compute_similarity_score(left_numbers: &Vec<i32>, right_numbers: &Vec<i32>) -> i32 {
    let mut right_numbers_count = HashMap::new();
    for &number in right_numbers {
        *right_numbers_count.entry(number).or_insert(0) += 1;
    }
    let mut similarity_score = 0;
    for &number in left_numbers {
        let count = right_numbers_count.get(&number).unwrap_or(&0);
        similarity_score += count * number;
    }
    similarity_score
}

fn main() {
    let (left_numbers, right_numbers) = extract_numbers("inputs/day01.txt");

    // Part 1
    let total_dist = compute_total_distance(&left_numbers, &right_numbers);
    println!("Total distance: {}", total_dist);

    // Part 2
    let similarity_score = compute_similarity_score(&left_numbers, &right_numbers);
    println!("Similarity score: {}", similarity_score);
}
