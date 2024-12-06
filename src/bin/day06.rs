use std::collections::HashSet;

use aoc2024::utils::file::read_file_to_grid;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Obstacle,
    Visited(HashSet<(i8, i8)>),
}

impl Cell {
    fn visited_from_dir(dir: (i8, i8)) -> Self {
        match dir {
            (0, 0) => panic!("Invalid direction"),
            (0, _) => Cell::Visited({
                let mut set = HashSet::new();
                set.insert(dir);
                set
            }),
            (_, 0) => Cell::Visited({
                let mut set = HashSet::new();
                set.insert(dir);
                set
            }),
            _ => panic!("Invalid direction"),
        }
    }

    fn update_visited(&mut self, dir: (i8, i8)) {
        match self {
            Cell::Visited(dirs) => {
                if dir == (0, 0) {
                    panic!("Invalid direction");
                }
                dirs.insert(dir);
            }
            _ => panic!("Cell is not visited"),
        }
    }

    fn is_visited(&self) -> bool {
        match self {
            Cell::Visited(..) => true,
            _ => false,
        }
    }

    fn is_visited_dir(&self, dir: (i8, i8)) -> bool {
        match self {
            Cell::Visited(dirs) => {
                if dir == (0, 0) {
                    panic!("Invalid direction");
                }
                dirs.contains(&dir)
            }
            _ => panic!("Cell is not visited"),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
    guard_pos: (usize, usize),
    guard_dir: (i8, i8),
}

impl Grid {
    fn from_file_grid(grid: Vec<Vec<char>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        let mut new_grid = vec![vec![Cell::Empty; width]; height];
        let mut guard_pos = (0, 0);
        let mut guard_dir = (0, 0);

        for (y, row) in grid.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                match ch {
                    '.' => new_grid[y][x] = Cell::Empty,
                    '#' => new_grid[y][x] = Cell::Obstacle,
                    '^' => {
                        guard_dir = (0, -1);
                        guard_pos = (x, y);
                        new_grid[y][x] = Cell::visited_from_dir(guard_dir);
                    }
                    _ => panic!("Invalid character in grid"),
                }
            }
        }

        assert!(guard_dir == (0, -1));

        Self {
            width,
            height,
            grid: new_grid,
            guard_pos,
            guard_dir,
        }
    }

    fn advance_guard(&mut self) -> (bool, bool) {
        let (x, y) = self.guard_pos;
        let (dx, dy) = self.guard_dir;
        let (nx, ny) = (x as i64 + dx as i64, y as i64 + dy as i64);
        if nx < 0 || nx >= self.width as i64 || ny < 0 || ny >= self.height as i64 {
            return (false, false);
        }
        let (nx, ny) = (nx as usize, ny as usize);

        let loop_detected = match self.grid[ny][nx] {
            Cell::Empty => {
                self.grid[ny][nx] = Cell::visited_from_dir(self.guard_dir);
                self.guard_pos = (nx, ny);
                false
            }
            Cell::Obstacle => {
                self.guard_dir = match self.guard_dir {
                    (0, -1) => (1, 0),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    (1, 0) => (0, 1),
                    _ => panic!("Invalid guard direction"),
                };
                self.grid[y][x].update_visited(self.guard_dir);
                false
            }
            Cell::Visited { .. } => {
                let has_loop = self.grid[ny][nx].is_visited_dir(self.guard_dir);
                self.grid[ny][nx].update_visited(self.guard_dir);
                self.guard_pos = (nx, ny);
                has_loop
            }
        };
        (true, loop_detected)
    }

    fn predict_full_guard_path(&mut self) {
        loop {
            let (advanced, _) = self.advance_guard();
            if !advanced {
                break;
            }
        }
    }

    fn predict_guard_path_until_loop(&mut self) -> bool {
        loop {
            let (advanced, loop_detected) = self.advance_guard();
            if !advanced {
                return false;
            }
            if loop_detected {
                return true;
            }
        }
    }

    fn num_visited_cells(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|&c| c.is_visited())
            .count()
    }

    fn add_obstacle(&mut self, pos: (usize, usize)) {
        self.grid[pos.1][pos.0] = Cell::Obstacle;
    }

    fn empty_cells(&self) -> EmptyCellsIterator {
        EmptyCellsIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

// Iterator for empty cells
struct EmptyCellsIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}

impl<'a> Iterator for EmptyCellsIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.y < self.grid.height {
            if self.x >= self.grid.width {
                self.x = 0;
                self.y += 1;
                continue;
            }
            let pos = (self.x, self.y);
            self.x += 1;
            if let Cell::Empty = self.grid.grid[pos.1][pos.0] {
                return Some(pos);
            }
        }
        None
    }
}

fn main() {
    // Part 1
    let grid = read_file_to_grid("inputs/day06.txt").expect("Failed to read file");
    let mut grid = Grid::from_file_grid(grid);
    grid.predict_full_guard_path();
    let num_visited = grid.num_visited_cells();

    println!("Number of visited cells: {}", num_visited);

    // Part 2
    let grid = read_file_to_grid("inputs/day06.txt").expect("Failed to read file");
    let grid = Grid::from_file_grid(grid);
    let mut num_loops = 0;
    for pos in grid.empty_cells() {
        let mut obstacle_grid = grid.clone();
        obstacle_grid.add_obstacle(pos);
        let loop_detected = obstacle_grid.predict_guard_path_until_loop();
        if loop_detected {
            num_loops += 1;
        }
    }

    println!("Number of loops: {}", num_loops);
}
