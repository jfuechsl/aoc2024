use std::{collections::HashMap, vec};

use aoc2024::utils::file::load_file_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumButton {
    Num(u8),
    A,
}

struct NumPathFinder(HashMap<(NumButton, NumButton), Vec<Vec<DirButton>>>);

impl Default for NumPathFinder {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl NumPathFinder {
    fn find_paths(&mut self, from: NumButton, to: NumButton) -> Vec<Vec<DirButton>> {
        if let Some(paths) = self.0.get(&(from, to)) {
            return paths.clone();
        }
        let (x1, y1) = from.to_coord();
        let (x2, y2) = to.to_coord();
        let dx = x2 as isize - x1 as isize;
        let dy = y2 as isize - y1 as isize;
        let num_horiz = dx.abs() as usize;
        let horiz_dir = if dx > 0 {
            DirButton::Right
        } else {
            DirButton::Left
        };
        let horiz = vec![horiz_dir; num_horiz];
        let num_vert = dy.abs() as usize;
        let vert_dir = if dy > 0 {
            DirButton::Down
        } else {
            DirButton::Up
        };
        let vert = vec![vert_dir; num_vert];
        let mut path_horiz_first: Vec<_> =
            horiz.iter().copied().chain(vert.iter().copied()).collect();
        path_horiz_first.push(DirButton::A);
        let mut path_vert_first: Vec<_> =
            vert.iter().copied().chain(horiz.iter().copied()).collect();
        path_vert_first.push(DirButton::A);
        let mut dirs = vec![];
        if x1 == 0 && y2 == 3 {
            dirs.push(path_horiz_first.clone());
        } else if y1 == 3 && x2 == 0 {
            dirs.push(path_vert_first.clone());
        } else {
            dirs.push(path_horiz_first.clone());
            if path_horiz_first != path_vert_first {
                dirs.push(path_vert_first.clone());
            }
        }
        self.0.insert((from, to), dirs.clone());
        dirs
    }
}

impl NumButton {
    fn to_coord(&self) -> (usize, usize) {
        match self {
            NumButton::Num(0) => (1, 3),
            NumButton::Num(1) => (0, 2),
            NumButton::Num(2) => (1, 2),
            NumButton::Num(3) => (2, 2),
            NumButton::Num(4) => (0, 1),
            NumButton::Num(5) => (1, 1),
            NumButton::Num(6) => (2, 1),
            NumButton::Num(7) => (0, 0),
            NumButton::Num(8) => (1, 0),
            NumButton::Num(9) => (2, 0),
            NumButton::A => (2, 3),
            _ => panic!("Invalid button"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirButton {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl DirButton {
    fn to_coord(&self) -> (usize, usize) {
        match self {
            DirButton::Up => (1, 0),
            DirButton::Left => (0, 1),
            DirButton::Down => (1, 1),
            DirButton::Right => (2, 1),
            DirButton::A => (2, 0),
        }
    }
}

struct DirPathFinder(HashMap<(DirButton, DirButton), Vec<Vec<DirButton>>>);

impl Default for DirPathFinder {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl DirPathFinder {
    fn find_paths(&mut self, from: DirButton, to: DirButton) -> Vec<Vec<DirButton>> {
        if let Some(paths) = self.0.get(&(from, to)) {
            return paths.clone();
        }
        let (x1, y1) = from.to_coord();
        let (x2, y2) = to.to_coord();
        let dx = x2 as isize - x1 as isize;
        let dy = y2 as isize - y1 as isize;
        let num_horiz = dx.abs() as usize;
        let horiz_dir = if dx > 0 {
            DirButton::Right
        } else {
            DirButton::Left
        };
        let horiz = vec![horiz_dir; num_horiz];
        let num_vert = dy.abs() as usize;
        let vert_dir = if dy > 0 {
            DirButton::Down
        } else {
            DirButton::Up
        };
        let vert = vec![vert_dir; num_vert];
        let mut path_horiz_first: Vec<_> =
            horiz.iter().copied().chain(vert.iter().copied()).collect();
        path_horiz_first.push(DirButton::A);
        let mut path_vert_first: Vec<_> =
            vert.iter().copied().chain(horiz.iter().copied()).collect();
        path_vert_first.push(DirButton::A);
        let mut dirs = vec![];
        if x1 == 0 && y2 == 0 {
            dirs.push(path_horiz_first.clone());
        } else if y1 == 0 && x2 == 0 {
            dirs.push(path_vert_first.clone());
        } else {
            dirs.push(path_horiz_first.clone());
            if path_horiz_first != path_vert_first {
                dirs.push(path_vert_first.clone());
            }
        }
        self.0.insert((from, to), dirs.clone());
        dirs
    }
}

struct Code {
    code: String,
    num_code: usize,
}

impl Code {
    fn from_string(code: &String) -> Self {
        let num_code: usize = code[..3].parse().unwrap();
        Self {
            code: code.clone(),
            num_code,
        }
    }

    fn compute_min_steps(&self, num_robots: usize) -> Option<usize> {
        Some(min_key_length(&self.code, num_robots))
    }

    fn compute_complexity(&self, num_robots: usize) -> Option<usize> {
        let min_steps = self.compute_min_steps(num_robots).unwrap();
        let complexity = min_steps * self.num_code;
        Some(complexity)
    }
}

fn build_code_paths(code: &String) -> Vec<Vec<Vec<DirButton>>> {
    let mut path_finder = NumPathFinder::default();
    let mut cur_key = NumButton::A;
    let mut paths = Vec::new();
    for c in code.chars() {
        let key = match c {
            'A' => NumButton::A,
            n if n.is_digit(10) => NumButton::Num(n.to_digit(10).unwrap() as u8),
            _ => panic!("Invalid code char"),
        };
        let path = path_finder.find_paths(cur_key, key);
        paths.push(path);
        cur_key = key;
    }
    paths
}

fn build_keys_rec(
    keys: &[DirButton],
    index: usize,
    prev_key: DirButton,
    curr_path: Vec<DirButton>,
    result: &mut Vec<Vec<DirButton>>,
    finder: &mut DirPathFinder,
) {
    if index == keys.len() {
        result.push(curr_path);
        return;
    }
    let curr_key = keys[index];
    let paths = finder.find_paths(prev_key, curr_key);
    for path in paths {
        let mut new_path = curr_path.clone();
        new_path.extend(path);
        build_keys_rec(keys, index + 1, curr_key, new_path, result, finder);
    }
}

fn find_shortest_keys(
    keys: Vec<DirButton>,
    depth: usize,
    cache: &mut HashMap<(Vec<DirButton>, usize), usize>,
    finder: &mut DirPathFinder,
) -> usize {
    if depth == 0 {
        return keys.len();
    }
    if let Some(&cached) = cache.get(&(keys.clone(), depth)) {
        return cached;
    }
    let mut total = 0;
    let mut sub_keys = Vec::new();
    let mut current_sub_key = Vec::new();
    for key in keys.iter().copied() {
        current_sub_key.push(key);
        if key == DirButton::A {
            if !current_sub_key.is_empty() {
                sub_keys.push(current_sub_key.clone());
                current_sub_key.clear();
            }
        }
    }
    if !current_sub_key.is_empty() {
        sub_keys.push(current_sub_key);
    }
    for sub_key in sub_keys {
        let mut sequences = Vec::new();
        build_keys_rec(
            &sub_key,
            0,
            DirButton::A,
            Vec::new(),
            &mut sequences,
            finder,
        );
        let mut min_steps = usize::MAX;
        for sequence in sequences {
            let steps = find_shortest_keys(sequence, depth - 1, cache, finder);
            if steps < min_steps {
                min_steps = steps;
            }
        }
        total += min_steps;
    }
    cache.insert((keys, depth), total);
    total
}

fn min_key_length(code: &String, max_depth: usize) -> usize {
    let mut dir_finder = DirPathFinder::default();
    let mut cache = HashMap::new();

    let code_paths = build_code_paths(code);

    let mut count = 0;
    for code_point_seqs in code_paths {
        let mut min_steps = usize::MAX;
        for code_seq in code_point_seqs {
            let steps = find_shortest_keys(code_seq, max_depth, &mut cache, &mut dir_finder);
            if steps < min_steps {
                min_steps = steps;
            }
        }
        count += min_steps;
    }
    count
}

fn main() {
    let filename = "inputs/day21.txt";
    let code_strings = load_file_lines(filename).expect("Invalid filename");

    // Part 1
    let num_robots = 2;
    let codes = code_strings.iter().map(Code::from_string);
    let complexities: Vec<_> = codes
        .map(|code| code.compute_complexity(num_robots).unwrap())
        .collect();
    let total_complexity: usize = complexities.iter().sum();
    assert_eq!(total_complexity, 176650);
    println!("Total complexity (2 robots): {}", total_complexity);

    // Part 2
    let num_robots = 25;
    let codes = code_strings.iter().map(Code::from_string);
    let complexities: Vec<_> = codes
        .map(|code| code.compute_complexity(num_robots).unwrap())
        .collect();
    let total_complexity: usize = complexities.iter().sum();
    assert_eq!(total_complexity, 217698355426872);
    println!("Total complexity (25 robots): {}", total_complexity);
}
