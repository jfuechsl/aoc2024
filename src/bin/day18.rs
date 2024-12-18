use std::collections::BTreeMap;

use aoc2024::utils::file::load_file_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Byte {
    Free,
    Corrupted,
}

#[derive(Debug, Clone)]
struct Memory {
    width: usize,
    height: usize,
    grid: Vec<Vec<Byte>>,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
}

impl Memory {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![Byte::Free; width]; height],
            start_pos: (0, 0),
            end_pos: (width - 1, height - 1),
        }
    }

    fn corrupt_at(&mut self, x: usize, y: usize) {
        self.grid[y][x] = Byte::Corrupted;
    }

    fn heuristic_distance(&self, (x, y): (usize, usize)) -> usize {
        let (tx, ty) = self.end_pos;
        let dx = (tx as isize - x as isize).abs() as usize;
        let dy = (ty as isize - y as isize).abs() as usize;
        dx + dy
    }

    fn find_shortest_path(&self) -> Option<usize> {
        let mut path_found = false;
        let mut positions = BTreeMap::new();
        let mut visited_steps = vec![vec![usize::MAX; self.width]; self.height];
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
                    && self.grid[y][x - 1] != Byte::Corrupted
                    && visited_steps[y][x - 1] > steps + 1
                {
                    next_positions.push(((x - 1, y), steps + 1));
                }
                if x < self.width - 1
                    && self.grid[y][x + 1] != Byte::Corrupted
                    && visited_steps[y][x + 1] > steps + 1
                {
                    next_positions.push(((x + 1, y), steps + 1));
                }
                if y > 0
                    && self.grid[y - 1][x] != Byte::Corrupted
                    && visited_steps[y - 1][x] > steps + 1
                {
                    next_positions.push(((x, y - 1), steps + 1));
                }
                if y < self.height - 1
                    && self.grid[y + 1][x] != Byte::Corrupted
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
        if path_found {
            Some(visited_steps[self.end_pos.1][self.end_pos.0])
        } else {
            None
        }
    }
}

fn main() {
    let filename = "inputs/day18.txt";
    let lines = load_file_lines(filename).expect("Failed to load input");
    let corrupt_coordinates = lines
        .into_iter()
        .map(|line| {
            let mut split = line.split(',');
            let x = split.next().unwrap().parse::<usize>().unwrap();
            let y = split.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    // Part 1
    let size = 71;
    let mut memory = Memory::new(size, size);
    for i in 0..1024 {
        let (x, y) = corrupt_coordinates[i];
        memory.corrupt_at(x, y);
    }
    let shortest_path = memory.find_shortest_path().unwrap();
    println!("Part 1: {}", shortest_path);

    // Part 2
    for i in 1024..corrupt_coordinates.len() {
        let (x, y) = corrupt_coordinates[i];
        memory.corrupt_at(x, y);
        if let None = memory.find_shortest_path() {
            println!("Part 2: {},{}", x, y);
            break;
        }
    }
}
