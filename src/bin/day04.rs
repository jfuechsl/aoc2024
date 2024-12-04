use aoc2024::utils::file::read_file_to_grid;
use regex::Regex;

fn transform_rows_to_columns(rows: &Vec<String>) -> Vec<String> {
    if rows.is_empty() {
        return vec![];
    }

    let num_columns = rows[0].len();
    let mut columns = vec![String::new(); num_columns];

    for row in rows {
        for (i, ch) in row.chars().enumerate() {
            columns[i].push(ch);
        }
    }

    columns
}

fn transform_rows_to_diagonals(rows: &Vec<String>, direction: i8) -> Vec<String> {
    if rows.is_empty() {
        return vec![];
    }

    let num_rows = rows.len();
    let num_columns = rows[0].len();
    let mut diagonals = vec![String::new(); num_rows + num_columns - 1];

    if direction >= 0 {
        for (i, row) in rows.iter().enumerate() {
            for (j, ch) in row.chars().enumerate() {
                diagonals[i + j].push(ch);
            }
        }
    } else {
        for (i, row) in rows.iter().enumerate() {
            for (j, ch) in row.chars().enumerate() {
                diagonals[num_columns - 1 + i - j].push(ch);
            }
        }
    }

    diagonals
}

fn reverse_lines(lines: &Vec<String>) -> Vec<String> {
    lines
        .iter()
        .map(|line| line.chars().rev().collect())
        .collect()
}

fn extract_xmas_count(text: &String) -> usize {
    let re = Regex::new(r"XMAS").expect("Invalid regex pattern");
    re.find_iter(text.as_str()).count()
}

fn has_xmas_pattern(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    assert!(row > 0);
    assert!(row < grid.len() - 1);
    assert!(col > 0);
    assert!(col < grid[0].len() - 1);
    if grid[row][col] != 'A' {
        return false;
    }
    // Main diagonal
    let main_diag_match = (grid[row - 1][col - 1] == 'M' && grid[row + 1][col + 1] == 'S')
        || (grid[row - 1][col - 1] == 'S' && grid[row + 1][col + 1] == 'M');
    // Secondary diagonal
    let sec_diag_match = (grid[row - 1][col + 1] == 'M' && grid[row + 1][col - 1] == 'S')
        || (grid[row - 1][col + 1] == 'S' && grid[row + 1][col - 1] == 'M');
    main_diag_match && sec_diag_match
}

fn count_xmas_patterns(grid: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for row in 1..grid.len() - 1 {
        for col in 1..grid[0].len() - 1 {
            if has_xmas_pattern(grid, row, col) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    // Part 1
    let rows = aoc2024::utils::file::load_file_lines("inputs/day04.txt")
        .expect("Failed to read input file");
    let rows_rev = reverse_lines(&rows);
    let columns = transform_rows_to_columns(&rows);
    let columns_rev = reverse_lines(&columns);
    let diagonals1 = transform_rows_to_diagonals(&rows, 1);
    let diagonals1_rev = reverse_lines(&diagonals1);
    let diagonals2 = transform_rows_to_diagonals(&rows, -1);
    let diagonals2_rev = reverse_lines(&diagonals2);

    let num_xmas: usize = rows
        .iter()
        .chain(rows_rev.iter())
        .chain(columns.iter())
        .chain(columns_rev.iter())
        .chain(diagonals1.iter())
        .chain(diagonals1_rev.iter())
        .chain(diagonals2.iter())
        .chain(diagonals2_rev.iter())
        .map(|line| extract_xmas_count(line))
        .sum();
    println!("Number of XMAS: {}", num_xmas);

    // Part 2
    let xmas_grid = read_file_to_grid("inputs/day04.txt").expect("Failed to read input file");
    let num_xmas_patterns = count_xmas_patterns(&xmas_grid);

    println!("Number of XMAS patterns: {}", num_xmas_patterns);
}
