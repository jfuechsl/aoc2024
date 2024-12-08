use std::collections::{HashMap, HashSet};

use aoc2024::utils::file::read_file_to_grid;

type Pos = (i64, i64);

struct AntennaGrid {
    width: i64,
    height: i64,
    antenna_positions: HashMap<char, Vec<Pos>>,
    antinode_positions: HashSet<Pos>,
}

impl AntennaGrid {
    fn from_char_grid(grid: Vec<Vec<char>>) -> Self {
        let width = grid[0].len() as i64;
        let height = grid.len() as i64;
        let mut antenna_positions = HashMap::new();
        let antinode_positions = HashSet::new();

        for y in 0..height {
            for x in 0..width {
                let c = grid[y as usize][x as usize];
                if c != '.' {
                    antenna_positions
                        .entry(c)
                        .or_insert(Vec::new())
                        .push((x, y));
                }
            }
        }

        Self {
            width,
            height,
            antenna_positions,
            antinode_positions,
        }
    }

    fn reset(&mut self) {
        self.antinode_positions.clear();
    }

    fn is_in_grid(&self, pos: Pos) -> bool {
        let (x, y) = pos;
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn compute_antinodes_part1(&self, pos1: Pos, pos2: Pos) -> Vec<Pos> {
        let (x1, y1) = pos1;
        let (x2, y2) = pos2;
        let dir_x = x2 - x1;
        let dir_y = y2 - y1;
        let an1_x = x2 + dir_x;
        let an1_y = y2 + dir_y;
        let an2_x = x1 - dir_x;
        let an2_y = y1 - dir_y;
        let an1 = (an1_x, an1_y);
        let an2 = (an2_x, an2_y);
        let mut ans = Vec::new();
        if self.is_in_grid(an1) {
            ans.push(an1);
        }
        if self.is_in_grid(an2) {
            ans.push(an2);
        }
        ans
    }

    fn compute_antinodes_part2(&self, pos1: Pos, pos2: Pos) -> Vec<Pos> {
        let (x1, y1) = pos1;
        let (x2, y2) = pos2;
        let dir_x = x2 - x1;
        let dir_y = y2 - y1;
        let mut ans = Vec::new();
        let mut per_mul = 0;
        loop {
            let an1_x = x2 + dir_x * per_mul;
            let an1_y = y2 + dir_y * per_mul;
            let an2_x = x1 - dir_x * per_mul;
            let an2_y = y1 - dir_y * per_mul;
            let an1 = (an1_x, an1_y);
            let an1_in = self.is_in_grid(an1);
            if an1_in {
                ans.push(an1);
            }
            let an2 = (an2_x, an2_y);
            let an2_in = self.is_in_grid(an2);
            if an2_in {
                ans.push(an2);
            }
            per_mul += 1;
            if !an1_in && !an2_in {
                break;
            }
        }
        ans
    }

    fn compute_antinodes_for_antennas(&self, positions: &Vec<Pos>, part1: bool) -> Vec<Pos> {
        assert!(positions.len() >= 2);
        let mut antinodes = Vec::new();
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let pos1 = positions[i];
                let pos2 = positions[j];
                let new_antinodes = if part1 {
                    self.compute_antinodes_part1(pos1, pos2)
                } else {
                    self.compute_antinodes_part2(pos1, pos2)
                };
                antinodes.extend(new_antinodes);
            }
        }
        antinodes
    }

    fn compute_all_antinodes(&mut self, part1: bool) {
        for positions in self.antenna_positions.values() {
            let antinodes = self.compute_antinodes_for_antennas(positions, part1);
            for antinode in antinodes {
                self.antinode_positions.insert(antinode);
            }
        }
    }

    fn num_antinodes(&self) -> usize {
        self.antinode_positions.len()
    }
}

fn main() {
    let filename = "inputs/day08.txt";
    let grid = read_file_to_grid(filename).expect("Failed to read file");
    let mut grid = AntennaGrid::from_char_grid(grid);

    // Part 1
    grid.compute_all_antinodes(true);
    let num_antinodes = grid.num_antinodes();
    println!("Number of antinodes (part 1): {}", num_antinodes);

    // Part 2
    grid.reset();
    grid.compute_all_antinodes(false);
    let num_antinodes = grid.num_antinodes();
    println!("Number of antinodes (part 2): {}", num_antinodes);
}
