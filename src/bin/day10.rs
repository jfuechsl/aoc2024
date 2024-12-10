use std::collections::HashSet;

use aoc2024::utils::file::read_file_to_grid;

#[derive(Debug)]
enum Waypoint {
    Branch(Vec<Waypoint>),
    Target(usize, usize),
    DeadEnd,
}

impl Waypoint {
    fn collect_target_positions_rec(&self, target_positions: &mut HashSet<(usize, usize)>) {
        match self {
            Waypoint::Branch(branches) => {
                for branch in branches {
                    branch.collect_target_positions_rec(target_positions);
                }
            }
            Waypoint::Target(x, y) => {
                target_positions.insert((*x, *y));
            }
            Waypoint::DeadEnd => {}
        }
    }

    fn score(&self) -> u32 {
        let mut target_positions = HashSet::new();
        self.collect_target_positions_rec(&mut target_positions);
        target_positions.len() as u32
    }

    fn rating(&self) -> u32 {
        match self {
            Waypoint::Branch(branches) => {
                let mut rating = 0;
                for branch in branches {
                    rating += branch.rating();
                }
                rating
            }
            Waypoint::Target(..) => 1,
            Waypoint::DeadEnd => 0,
        }
    }
}

struct TopoMap {
    width: usize,
    height: usize,
    grid: Vec<Vec<u8>>,
    start_positions: Vec<(usize, usize)>,
}

impl TopoMap {
    fn from_grid(grid: Vec<Vec<char>>) -> Self {
        let mut topo_grid = Vec::new();
        let mut start_positions = Vec::new();
        let width = grid[0].len();
        let height = grid.len();

        for (y, row) in grid.iter().enumerate() {
            let mut topo_row = Vec::new();
            for (x, ch) in row.iter().enumerate() {
                let height = ch.to_digit(10).expect("Invalid height") as u8;
                topo_row.push(height);
                if height == 0 {
                    start_positions.push((x, y));
                }
            }
            topo_grid.push(topo_row);
        }

        Self {
            width,
            height,
            grid: topo_grid,
            start_positions,
        }
    }

    fn viable_next_steps(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let height = self.grid[y][x];
        if height == 9 {
            return Vec::new();
        }
        let next_height = height + 1;
        let mut next_steps = Vec::new();
        if x > 0 && self.grid[y][x - 1] == next_height {
            next_steps.push((x - 1, y));
        }
        if x < self.width - 1 && self.grid[y][x + 1] == next_height {
            next_steps.push((x + 1, y));
        }
        if y > 0 && self.grid[y - 1][x] == next_height {
            next_steps.push((x, y - 1));
        }
        if y < self.height - 1 && self.grid[y + 1][x] == next_height {
            next_steps.push((x, y + 1));
        }
        next_steps
    }

    fn is_target(&self, (x, y): (usize, usize)) -> bool {
        self.grid[y][x] == 9
    }

    fn next_waypoint(&self, (x, y): (usize, usize)) -> Waypoint {
        if self.is_target((x, y)) {
            return Waypoint::Target(x, y);
        }
        let next_steps = self.viable_next_steps((x, y));
        if next_steps.is_empty() {
            return Waypoint::DeadEnd;
        }
        let mut branches = Vec::new();
        for next_step in next_steps {
            branches.push(self.next_waypoint(next_step));
        }
        Waypoint::Branch(branches)
    }

    fn trailhead_waypoints(&self) -> Vec<Waypoint> {
        let mut waypoints = Vec::new();
        for start_position in &self.start_positions {
            let waypoint = self.next_waypoint(*start_position);
            waypoints.push(waypoint);
        }
        waypoints
    }

    fn sum_of_trailhead_scores(&self) -> u32 {
        self.trailhead_waypoints().iter().map(|w| w.score()).sum()
    }

    fn sum_of_trailhead_ratings(&self) -> u32 {
        self.trailhead_waypoints().iter().map(|w| w.rating()).sum()
    }
}

fn main() {
    let filename = "inputs/day10.txt";
    let topo_grid = read_file_to_grid(filename).expect("Failed to read file");
    let topo_map = TopoMap::from_grid(topo_grid);

    // Part 1
    let sum_scores = topo_map.sum_of_trailhead_scores();
    assert_eq!(sum_scores, 461);
    println!("Sum of trailhead scores: {}", sum_scores);

    // Part 2
    let sum_ratings = topo_map.sum_of_trailhead_ratings();
    assert_eq!(sum_ratings, 875);
    println!("Sum of trailhead ratings: {}", sum_ratings);
}
