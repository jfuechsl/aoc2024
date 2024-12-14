use std::collections::HashMap;

use aoc2024::utils::file::load_file_lines;

struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

fn parse_robots(input: &Vec<String>) -> Vec<Robot> {
    input
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let position_part = parts[0]
                .trim_start_matches("p=")
                .split(',')
                .collect::<Vec<&str>>();
            let velocity_part = parts[1]
                .trim_start_matches("v=")
                .split(',')
                .collect::<Vec<&str>>();
            let position = (
                position_part[0].parse::<i64>().unwrap(),
                position_part[1].parse::<i64>().unwrap(),
            );
            let velocity = (
                velocity_part[0].parse::<i64>().unwrap(),
                velocity_part[1].parse::<i64>().unwrap(),
            );
            Robot { position, velocity }
        })
        .collect()
}

struct Tiles {
    robots: Vec<Robot>,
    width: i64,
    height: i64,
}

impl Tiles {
    fn new(robots: Vec<Robot>, width: i64, height: i64) -> Tiles {
        Tiles {
            robots,
            width,
            height,
        }
    }

    fn predict_positions_after(&mut self, seconds: i64) {
        for robot in self.robots.iter_mut() {
            robot.position = (
                (robot.position.0 + robot.velocity.0 * seconds) % self.width,
                (robot.position.1 + robot.velocity.1 * seconds) % self.height,
            );
            if robot.position.0 < 0 {
                robot.position.0 = self.width + robot.position.0;
            }
            if robot.position.1 < 0 {
                robot.position.1 = self.height + robot.position.1;
            }
        }
    }

    fn pos_to_quadrant(&self, (x, y): (usize, usize)) -> Option<usize> {
        let mx = ((self.width - 1) / 2) as usize;
        let my = ((self.height - 1) / 2) as usize;
        if x < mx {
            // Quadrant 1 or 3
            if y < my {
                return Some(1);
            } else if y > my {
                return Some(3);
            }
        } else if x > mx {
            // Quadrant 2 or 4
            if y < my {
                return Some(2);
            } else if y > my {
                return Some(4);
            }
        }
        None
    }

    fn counts_per_quadrant(&self) -> HashMap<usize, usize> {
        let mut counts = HashMap::new();
        for robot in self.robots.iter() {
            let pos = (robot.position.0 as usize, robot.position.1 as usize);
            if let Some(quadrant) = self.pos_to_quadrant(pos) {
                let count = counts.entry(quadrant).or_insert(0);
                *count += 1;
            }
        }
        counts
    }

    fn highest_quadrant_concentration(&self) -> f64 {
        let counts = self.counts_per_quadrant();
        let max_counts_per_quadrant = *counts.values().max().unwrap();
        max_counts_per_quadrant as f64 / self.robots.len() as f64
    }
}

fn main() {
    let filename = "inputs/day14.txt";
    let input = load_file_lines(filename).expect("Failed to read file");

    // Part 1
    let mut tiles = Tiles::new(parse_robots(&input), 101, 103);
    tiles.predict_positions_after(100);
    let counts = tiles.counts_per_quadrant();
    let safety_score = counts.values().product::<usize>();
    assert_eq!(safety_score, 222062148);
    println!("Safety score: {}", safety_score);

    // Part 2
    // Assumption: If the concentration of robots in a quadrant is higher than 50%, they form a Christmas tree.
    let mut tiles = Tiles::new(parse_robots(&input), 101, 103);
    let mut seconds = 0;
    loop {
        tiles.predict_positions_after(1);
        seconds += 1;
        let concentration = tiles.highest_quadrant_concentration();
        if concentration > 0.5 {
            println!(
                "Seconds to converge: {} (concentration = {})",
                seconds, concentration
            );
            break;
        }
    }
    assert_eq!(seconds, 7520);
}
