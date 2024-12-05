use std::collections::HashMap;

use aoc2024::utils::file::load_file_lines;

fn split_rules_and_updates(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut is_update = false;

    for line in lines {
        if line.trim().is_empty() {
            is_update = true;
            continue;
        }
        if is_update {
            updates.push(line);
        } else {
            rules.push(line);
        }
    }

    (rules, updates)
}

fn parse_rules(rules: Vec<String>) -> Vec<(usize, usize)> {
    rules
        .iter()
        .map(|rule| {
            let parts: Vec<&str> = rule.split("|").collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect()
}

fn parse_updates(updates: Vec<String>) -> Vec<HashMap<usize, usize>> {
    updates
        .iter()
        .map(|update| {
            update
                .split(",")
                .enumerate()
                .map(|(i, p)| (p.parse().unwrap(), i))
                .collect()
        })
        .collect()
}

fn is_valid_update(rules: &Vec<(usize, usize)>, update: &HashMap<usize, usize>) -> bool {
    for (p1, p2) in rules {
        if update.contains_key(p1) && update.contains_key(p2) {
            if update[p1] > update[p2] {
                return false;
            }
        }
    }
    true
}

fn correct_update(rules: &Vec<(usize, usize)>, update: &mut HashMap<usize, usize>) {
    loop {
        for (p1, p2) in rules {
            if update.contains_key(p1) && update.contains_key(p2) {
                if update[p1] > update[p2] {
                    let tmp = update[p1];
                    update.insert(*p1, update[p2]);
                    update.insert(*p2, tmp);
                }
            }
        }
        if is_valid_update(rules, update) {
            break;
        }
    }
}

fn extract_middle_page(update: HashMap<usize, usize>) -> usize {
    let mut us: Vec<(usize, usize)> = update.into_iter().collect();
    us.sort_by_key(|(_, i)| *i);
    assert!(us.len() % 2 == 1, "Invalid update length {:?}", us);
    let middle = us.len() / 2 + 1 - 1;
    us[middle].0
}

fn main() {
    let lines = load_file_lines("inputs/day05.txt").expect("File not found");
    let (rules, updates) = split_rules_and_updates(lines);
    let rules = parse_rules(rules);
    let updates = parse_updates(updates);

    // Part 1
    let sum_middle_pages_of_valid_updates: usize = updates
        .clone()
        .into_iter()
        .filter(|update| is_valid_update(&rules, update))
        .map(extract_middle_page)
        .sum();

    println!(
        "Sum of middle pages of valid updates: {}",
        sum_middle_pages_of_valid_updates
    );

    // Part 2
    let sum_middle_pages_of_corrected_updates: usize = updates
        .into_iter()
        .filter(|update| !is_valid_update(&rules, update))
        .map(|mut update| {
            correct_update(&rules, &mut update);
            extract_middle_page(update)
        })
        .sum();

    println!(
        "Sum of middle pages of corrected updates: {}",
        sum_middle_pages_of_corrected_updates
    );
}
