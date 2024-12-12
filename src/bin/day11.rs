use std::collections::HashMap;

use aoc2024::utils::file::read_lines;

type StonesMap = HashMap<i64, usize>;

fn split_stone_val(sval: i64) -> Option<(i64, i64)> {
    let sval_str = sval.to_string();
    let num_digits = sval_str.len() as i64;
    if num_digits % 2 == 0 {
        let half = num_digits / 2;
        let slval = sval_str[..half as usize]
            .parse::<i64>()
            .expect("Failed to parse left half");
        let srval = sval_str[half as usize..]
            .parse::<i64>()
            .expect("Failed to parse right half");
        Some((slval, srval))
    } else {
        None
    }
}

fn blink(stones: &mut StonesMap) {
    let mut new_stones = HashMap::new();
    stones.into_iter().for_each(|(sval, num)| {
        if *sval == 0 {
            *new_stones.entry(1).or_insert(0) += *num;
        } else if let Some((slval, srval)) = split_stone_val(*sval) {
            *new_stones.entry(slval).or_insert(0) += *num;
            *new_stones.entry(srval).or_insert(0) += *num;
        } else {
            *new_stones.entry(*sval * 2024).or_insert(0) += *num;
        }
    });
    *stones = new_stones;
}

fn main() {
    let filename = "inputs/day11.txt";
    let stones_string = read_lines(filename).expect("Failed to read file");
    let orig_stones: StonesMap = stones_string
        .split(' ')
        .map(|s| {
            s.trim()
                .parse::<i64>()
                .expect(format!("Failed to parse '{}'", s).as_str())
        })
        .map(|i| (i, 1))
        .collect();

    // Part 1
    let mut stones = orig_stones.clone();
    for _ in 0..25 {
        blink(&mut stones);
    }
    let num_stones: usize = stones.values().sum();
    assert_eq!(num_stones, 189167);
    println!("Number of stones (after 25 blinks): {}", num_stones);

    // Part 2
    let mut stones = orig_stones.clone();
    for _ in 0..75 {
        blink(&mut stones);
    }
    let num_stones: usize = stones.values().sum();
    assert_eq!(num_stones, 225253278506288);
    println!("Number of stones (after 75 blinks): {}", num_stones);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_stone_val() {
        assert_eq!(split_stone_val(1000), Some((10, 0)));
        // Add more test cases as needed
    }
}
