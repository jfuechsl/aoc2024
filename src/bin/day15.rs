use std::collections::HashSet;

use aoc2024::utils::file::load_file_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

impl Movement {
    fn horizontal(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridCell {
    Wall,
    Open,
    Package,
    WPackageL,
    WPackageR,
}

struct Grid {
    grid: Vec<Vec<GridCell>>,
    robot_position: (i64, i64),
}

impl Grid {
    fn from_grid_lines(lines: &Vec<String>, wide: bool) -> Self {
        let mut grid = Vec::new();
        let mut robot_position = None;
        for (y, line) in lines.iter().enumerate() {
            let mut row = Vec::new();
            for (xh, ch) in line.chars().enumerate() {
                let x = if wide { 2 * xh } else { xh };
                let cells = match ch {
                    '#' if !wide => vec![GridCell::Wall],
                    'O' if !wide => vec![GridCell::Package],
                    '.' if !wide => vec![GridCell::Open],
                    '#' if wide => vec![GridCell::Wall, GridCell::Wall],
                    'O' if wide => vec![GridCell::WPackageL, GridCell::WPackageR],
                    '.' if wide => vec![GridCell::Open, GridCell::Open],
                    '@' => {
                        robot_position = Some((x as i64, y as i64));
                        if wide {
                            vec![GridCell::Open, GridCell::Open]
                        } else {
                            vec![GridCell::Open]
                        }
                    }
                    _ => panic!("Invalid character in grid: {}", ch),
                };
                row.extend(cells);
            }
            grid.push(row);
        }
        let robot_position = robot_position.expect("Robot position not found");
        Self {
            grid,
            robot_position,
        }
    }

    fn search_affected_packages(
        &self,
        movement: Movement,
    ) -> Option<((i64, i64), Vec<((i64, i64), GridCell)>)> {
        let (x, y) = self.robot_position;
        let (dx, dy) = match movement {
            Movement::Up => (0, -1),
            Movement::Down => (0, 1),
            Movement::Left => (-1, 0),
            Movement::Right => (1, 0),
        };
        let mut affected_packages = Vec::new();
        let mut front = HashSet::from_iter([(x, y)]);
        let mut can_move = true;
        'outer: loop {
            let mut new_front = HashSet::new();
            for (fx, fy) in front.iter() {
                let new_fx = fx + dx;
                let new_fy = fy + dy;
                match self.grid[new_fy as usize][new_fx as usize] {
                    GridCell::Wall => {
                        can_move = false;
                        break 'outer;
                    }
                    GridCell::Open => {}
                    GridCell::Package => {
                        affected_packages.push(((new_fx, new_fy), GridCell::Package));
                        new_front.insert((new_fx, new_fy));
                    }
                    GridCell::WPackageL => {
                        if movement.horizontal() {
                            affected_packages.push(((new_fx, new_fy), GridCell::WPackageL));
                            new_front.insert((new_fx, new_fy));
                        } else {
                            assert!(
                                self.grid[new_fy as usize][(new_fx + 1) as usize]
                                    == GridCell::WPackageR
                            );
                            affected_packages.push(((new_fx, new_fy), GridCell::WPackageL));
                            affected_packages.push(((new_fx + 1, new_fy), GridCell::WPackageR));
                            new_front.insert((new_fx, new_fy));
                            new_front.insert((new_fx + 1, new_fy));
                        }
                    }
                    GridCell::WPackageR => {
                        if movement.horizontal() {
                            affected_packages.push(((new_fx, new_fy), GridCell::WPackageR));
                            new_front.insert((new_fx, new_fy));
                        } else {
                            assert!(
                                self.grid[new_fy as usize][(new_fx - 1) as usize]
                                    == GridCell::WPackageL
                            );
                            affected_packages.push(((new_fx, new_fy), GridCell::WPackageR));
                            affected_packages.push(((new_fx - 1, new_fy), GridCell::WPackageL));
                            new_front.insert((new_fx, new_fy));
                            new_front.insert((new_fx - 1, new_fy));
                        }
                    }
                }
            }
            if new_front.is_empty() {
                break;
            }
            front = new_front;
        }
        if can_move {
            Some(((dx, dy), affected_packages))
        } else {
            None
        }
    }

    fn move_robot(&mut self, movement: Movement) {
        if let Some(((dx, dy), affected_packages)) = self.search_affected_packages(movement) {
            let (x, y) = self.robot_position;
            let new_x = x + dx;
            let new_y = y + dy;
            for ((package_x, package_y), _) in affected_packages.iter() {
                self.grid[*package_y as usize][*package_x as usize] = GridCell::Open;
            }
            for ((package_x, package_y), cell) in affected_packages.iter() {
                let px = (package_x + dx) as usize;
                let py = (package_y + dy) as usize;
                self.grid[py][px] = *cell;
            }
            self.robot_position = (new_x, new_y);
        }
    }

    fn package_coordinates(&self) -> Vec<usize> {
        let mut package_coordinates = Vec::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if matches!(*cell, GridCell::Package | GridCell::WPackageL) {
                    package_coordinates.push(y * 100 + x);
                }
            }
        }
        package_coordinates
    }

    #[allow(dead_code)]
    fn print(&self) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if self.robot_position == (x as i64, y as i64) {
                    // assert!(matches!(cell, GridCell::Open));
                    print!("@");
                    continue;
                }
                let ch = match cell {
                    GridCell::Wall => '#',
                    GridCell::Open => '.',
                    GridCell::Package => 'O',
                    GridCell::WPackageL => '[',
                    GridCell::WPackageR => ']',
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

fn main() {
    let filename = "inputs/day15.txt";
    let lines = load_file_lines(filename).expect("Could not load file");
    let mut sections = lines.split(|l| l.is_empty());
    let grid_lines = sections.next().expect("No grid found").into();
    let movements = sections
        .next()
        .expect("No movement found")
        .iter()
        .flat_map(|l| l.chars())
        .map(|ch| match ch {
            '^' => Movement::Up,
            'v' => Movement::Down,
            '<' => Movement::Left,
            '>' => Movement::Right,
            _ => panic!("Invalid movement character: {}", ch),
        })
        .collect::<Vec<_>>();

    // Part 1
    let mut grid = Grid::from_grid_lines(&grid_lines, false);
    for movement in &movements {
        grid.move_robot(*movement);
    }
    let sum_gps_coords: usize = grid.package_coordinates().iter().sum();
    assert_eq!(sum_gps_coords, 1516281);
    println!("Sum of GPS coordinates (warehouse 1): {}", sum_gps_coords);

    // Part 2
    let mut grid = Grid::from_grid_lines(&grid_lines, true);
    for movement in &movements {
        grid.move_robot(*movement);
    }
    let sum_gps_coords: usize = grid.package_coordinates().iter().sum();
    assert_eq!(sum_gps_coords, 1527969);
    println!("Sum of GPS coordinates (warehouse 2): {}", sum_gps_coords);
}
