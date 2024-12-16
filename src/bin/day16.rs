use std::collections::{BTreeMap, HashSet};

use aoc2024::utils::file::read_file_to_grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    RotateLeft,
    RotateRight,
    Forward,
}

impl Action {
    fn incremental_score(&self) -> i64 {
        match self {
            Action::RotateLeft | Action::RotateRight => 1000,
            Action::Forward => 1,
        }
    }

    fn is_rotation(&self) -> bool {
        match self {
            Action::RotateLeft | Action::RotateRight => true,
            Action::Forward => false,
        }
    }

    fn is_forward(&self) -> bool {
        match self {
            Action::RotateLeft | Action::RotateRight => false,
            Action::Forward => true,
        }
    }

    fn update_direction(&self, (dx, dy): (i64, i64)) -> (i64, i64) {
        match self {
            Action::RotateLeft => (dy, -dx),
            Action::RotateRight => (-dy, dx),
            Action::Forward => (dx, dy),
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
    actions: Vec<Action>,
    visited_points: HashSet<(i64, i64)>,
    score: i64,
    position: (i64, i64),
    direction: (i64, i64),
}

impl Path {
    fn new(position: (i64, i64), direction: (i64, i64)) -> Self {
        Path {
            actions: Vec::new(),
            visited_points: HashSet::from_iter([position]),
            score: 0,
            position,
            direction,
        }
    }

    fn add_action(&mut self, action: Action) {
        self.actions.push(action);
        self.score += action.incremental_score();
        self.direction = action.update_direction(self.direction);
        if action.is_forward() {
            self.position = (
                self.position.0 + self.direction.0,
                self.position.1 + self.direction.1,
            );
            self.visited_points.insert(self.position);
        }
    }

    fn prev_action(&self) -> Option<Action> {
        self.actions.last().copied()
    }

    fn key(&self) -> ((i64, i64), (i64, i64)) {
        (self.position, self.direction)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
}

struct Maze {
    grid: Vec<Vec<Tile>>,
    start_pos: (i64, i64),
    end_pos: (i64, i64),
}

impl Maze {
    fn from_grid(grid: &Vec<Vec<char>>) -> Self {
        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);
        let mut maze = Vec::new();
        for (y, row) in grid.iter().enumerate() {
            let mut maze_row = Vec::new();
            for (x, &cell) in row.iter().enumerate() {
                let tile = match cell {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'S' => {
                        start_pos = (x as i64, y as i64);
                        Tile::Empty
                    }
                    'E' => {
                        end_pos = (x as i64, y as i64);
                        Tile::Empty
                    }
                    _ => panic!("Invalid cell: {}", cell),
                };
                maze_row.push(tile);
            }
            maze.push(maze_row);
        }
        Maze {
            grid: maze,
            start_pos,
            end_pos,
        }
    }

    fn viable_actions(
        &self,
        pos: (i64, i64),
        dir: (i64, i64),
        prev_action: Option<Action>,
    ) -> Vec<Action> {
        let (x, y) = pos;
        let (dx, dy) = dir;
        let (nx, ny) = (x as i64 + dx, y as i64 + dy);
        let next_is_empty = self.grid[ny as usize][nx as usize] == Tile::Empty;
        let mut actions = Vec::new();
        if next_is_empty {
            actions.push(Action::Forward);
        }
        let prev_was_rotation = prev_action.map_or(false, |a| a.is_rotation());
        if prev_was_rotation {
            // Doesn't make sense to rotate twice in a row
            return actions;
        }
        // Rotation doesn't make sense if forward would not be a viable next action
        for action in [Action::RotateLeft, Action::RotateRight] {
            let (ndx, ndy) = action.update_direction(dir);
            let (nx, ny) = (x + ndx, y + ndy);
            let next_is_empty = self.grid[ny as usize][nx as usize] == Tile::Empty;
            if next_is_empty {
                actions.push(action);
            }
        }
        actions
    }

    fn find_paths(&self) -> (i64, Vec<Path>) {
        let mut finished_paths: Vec<Path> = Vec::new();
        let mut paths: BTreeMap<((i64, i64), (i64, i64)), Path> = BTreeMap::new();
        let mut best_score = 89460;
        let start_path = Path::new(self.start_pos, (1, 0));
        paths.insert(start_path.key(), start_path);
        loop {
            let mut new_paths: BTreeMap<((i64, i64), (i64, i64)), Path> = BTreeMap::new(); //paths.clone();
            let mut sorted_paths = paths.values().cloned().collect::<Vec<_>>();
            sorted_paths.sort_by_key(|p| p.score);
            for path in sorted_paths {
                let pos = path.position;
                let dir = path.direction;
                let prev_action = path.prev_action();
                for action in self.viable_actions(pos, dir, prev_action) {
                    let mut new_path = path.clone();
                    new_path.add_action(action);
                    if new_path.position == self.end_pos {
                        // Found the end
                        if new_path.score < best_score {
                            best_score = new_path.score;
                        }
                        finished_paths.push(new_path.clone());
                    }
                    if let Some(existing_path) = new_paths.get(&new_path.key()) {
                        if new_path.score == existing_path.score {
                            // If we have a path with the same score,
                            // we want to keep the visited points of both of them.
                            // This is a slight hack: later on we really only care
                            // about the totallity of the visited points of the best paths.
                            new_path.visited_points = new_path
                                .visited_points
                                .union(&existing_path.visited_points)
                                .copied()
                                .collect();
                            new_paths.insert(new_path.key(), new_path);
                        } else if new_path.score < existing_path.score {
                            new_paths.insert(new_path.key(), new_path);
                        }
                    } else if new_path.score <= best_score {
                        new_paths.insert(new_path.key(), new_path);
                    }
                }
            }
            paths = new_paths;
            let all_at_end = paths.values().all(|p| p.position == self.end_pos);
            if all_at_end {
                break;
            }
        }
        let best_paths = finished_paths
            .into_iter()
            .filter(|p| p.score == best_score)
            .collect::<Vec<_>>();
        (best_score, best_paths)
    }
}

fn main() {
    let filename = "inputs/day16.txt";
    let grid = read_file_to_grid(filename).expect("Failed to read file");

    let maze = Maze::from_grid(&grid);
    let (min_score, best_paths) = maze.find_paths();

    // Part 1
    assert_eq!(min_score, 89460);
    println!("The minimum score is: {}", min_score);

    // Part 2
    let num_tiles = best_paths
        .iter()
        .fold(HashSet::new(), |acc, p| {
            acc.union(&p.visited_points).copied().collect()
        })
        .len();
    assert_eq!(num_tiles, 504);
    println!("The number of tiles in the best paths are: {}", num_tiles);
}
