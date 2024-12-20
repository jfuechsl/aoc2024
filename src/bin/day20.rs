use std::collections::{BTreeMap, HashSet};

use aoc2024::utils::file::read_file_to_grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

type Pos = (usize, usize);

#[derive(Debug)]
struct Track {
    width: usize,
    height: usize,
    track: Vec<Vec<Tile>>,
    start_pos: Pos,
    end_pos: Pos,
}

impl Track {
    fn from_grid(grid: &Vec<Vec<char>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        let mut track = vec![vec![Tile::Empty; width]; height];
        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);
        for y in 0..height {
            for x in 0..width {
                match grid[y][x] {
                    '.' => track[y][x] = Tile::Empty,
                    '#' => track[y][x] = Tile::Wall,
                    'S' => {
                        track[y][x] = Tile::Empty;
                        start_pos = (x, y);
                    }
                    'E' => {
                        track[y][x] = Tile::Empty;
                        end_pos = (x, y);
                    }
                    _ => panic!("Invalid character in grid"),
                }
            }
        }
        Self {
            width,
            height,
            track,
            start_pos,
            end_pos,
        }
    }

    fn heuristic_distance(&self, (x, y): Pos) -> usize {
        let (tx, ty) = self.end_pos;
        let dx = (tx as isize - x as isize).abs() as usize;
        let dy = (ty as isize - y as isize).abs() as usize;
        dx + dy
    }

    fn build_path_map(&self) -> Vec<Vec<usize>> {
        let mut path_found = false;
        let mut positions = BTreeMap::new();
        let mut visited_steps = vec![vec![usize::MAX; self.width]; self.height];
        visited_steps[self.start_pos.1][self.start_pos.0] = 0;
        positions.insert(
            self.heuristic_distance(self.start_pos),
            vec![(self.start_pos, 0)],
        );
        while let Some((_, pos_steps)) = positions.pop_first() {
            for (pos, steps) in pos_steps {
                if pos == self.end_pos {
                    path_found = true;
                }
                let (x, y) = pos;
                let mut next_positions = Vec::new();
                if x > 0
                    && self.track[y][x - 1] != Tile::Wall
                    && visited_steps[y][x - 1] > steps + 1
                {
                    next_positions.push(((x - 1, y), steps + 1));
                }
                if x < self.width - 1
                    && self.track[y][x + 1] != Tile::Wall
                    && visited_steps[y][x + 1] > steps + 1
                {
                    next_positions.push(((x + 1, y), steps + 1));
                }
                if y > 0
                    && self.track[y - 1][x] != Tile::Wall
                    && visited_steps[y - 1][x] > steps + 1
                {
                    next_positions.push(((x, y - 1), steps + 1));
                }
                if y < self.height - 1
                    && self.track[y + 1][x] != Tile::Wall
                    && visited_steps[y + 1][x] > steps + 1
                {
                    next_positions.push(((x, y + 1), steps + 1));
                }
                for (np, ns) in next_positions {
                    let h = self.heuristic_distance(np);
                    let (nx, ny) = np;
                    visited_steps[ny][nx] = ns;
                    positions.entry(h).or_default().push((np, ns));
                }
            }
        }
        if !path_found {
            panic!("No path found");
        }
        visited_steps
    }

    fn free_positions(&self) -> Vec<Pos> {
        let mut free = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.track[y][x] == Tile::Empty {
                    free.push((x, y));
                }
            }
        }
        free
    }

    fn double_iter_free(&self, max_dist: usize) -> impl Iterator<Item = (Pos, Pos, usize)> + '_ {
        let free = self.free_positions();
        let mut evaluated: HashSet<(Pos, Pos)> = HashSet::new();
        let mut free_pairs: Vec<(Pos, Pos, usize)> = Vec::new();
        for p1 in free.iter() {
            for p2 in free.iter() {
                let p1 = *p1;
                let p2 = *p2;
                let (p1, p2) = sort_points(p1, p2);
                if evaluated.contains(&(p1, p2)) {
                    continue;
                }
                evaluated.insert((p1, p2));
                let (x1, y1) = p1;
                let (x2, y2) = p2;
                let dist = (x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs();
                if dist <= max_dist as isize {
                    free_pairs.push((p1, p2, dist as usize));
                }
            }
        }
        free_pairs.into_iter()
    }
}

fn sort_points(p1: Pos, p2: Pos) -> (Pos, Pos) {
    if p1 < p2 {
        (p1, p2)
    } else {
        (p2, p1)
    }
}

fn main() {
    let filename = "inputs/day20.txt";
    let grid = read_file_to_grid(filename).expect("Invalid filename");

    let track = Track::from_grid(&grid);
    let visited_steps = track.build_path_map();

    // Part 1
    let num_100_cheats = track
        .double_iter_free(2)
        .map(|(p1, p2, dist)| {
            let steps_diff = visited_steps[p1.1][p1.0].abs_diff(visited_steps[p2.1][p2.0]) as i64;
            let cheat_speedup = steps_diff - dist as i64;
            cheat_speedup
        })
        .filter(|speedup| *speedup >= 100)
        .count();
    assert_eq!(num_100_cheats, 1404);
    println!("Number of 100+ speedup cheats: {}", num_100_cheats);

    // Part 2
    let mut speedup_counts = BTreeMap::new();
    track
        .double_iter_free(20)
        .map(|(p1, p2, dist)| {
            let steps_diff = visited_steps[p1.1][p1.0].abs_diff(visited_steps[p2.1][p2.0]) as i64;
            let cheat_speedup = steps_diff - dist as i64;
            cheat_speedup
        })
        .filter(|speedup| *speedup >= 100)
        .for_each(|speedup| {
            *speedup_counts.entry(speedup).or_insert(0) += 1;
        });
    let num_100_speedups = speedup_counts.values().sum::<usize>();
    assert_eq!(num_100_speedups, 1010981);
    println!("Number of 100+ speedup 20-cheats: {}", num_100_speedups);
}
