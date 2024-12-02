use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn load_file_lines(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn parse_integers(input: &str) -> Option<Vec<i32>> {
    input
        .split_whitespace()
        .map(|part| part.parse::<i32>().ok())
        .collect()
}

fn is_safe_1(levels: &Vec<i32>) -> bool {
    assert!(levels.len() >= 3);
    for i in 2..levels.len() {
        let d1 = levels[i - 1] - levels[i - 2];
        let d2 = levels[i] - levels[i - 1];
        if d1 * d2 < 0 {
            return false;
        }
        if d1.abs() < 1 || d2.abs() < 1 {
            return false;
        }
        if d1.abs() > 3 || d2.abs() > 3 {
            return false;
        }
    }
    true
}

fn is_safe_2(levels: &Vec<i32>) -> bool {
    if is_safe_1(levels) {
        return true;
    } else {
        for i in 0..levels.len() {
            let mut new_levels = levels.clone();
            new_levels.remove(i);
            if is_safe_1(&new_levels) {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let reports = load_file_lines("inputs/day02.txt")
        .expect("Could not load file")
        .iter()
        .map(|line| parse_integers(line).expect("Could not parse line"))
        .collect::<Vec<Vec<i32>>>();
    let num_safe_1 = reports
        .iter()
        .map(|report| if is_safe_1(report) { 1 } else { 0 })
        .sum::<i32>();
    println!("Number of safe reports (version 1): {}", num_safe_1);

    let num_safe_2 = reports
        .iter()
        .map(|report| if is_safe_2(report) { 1 } else { 0 })
        .sum::<i32>();
    println!("Number of safe reports (version 2): {}", num_safe_2);
}
