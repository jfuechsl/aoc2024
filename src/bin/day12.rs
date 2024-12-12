use std::collections::{BTreeMap, HashSet};

use aoc2024::utils::file::read_file_to_grid;

type PlotsSet = HashSet<(i64, i64, char)>;

struct Region {
    plot_type: char,
    plots: PlotsSet,
}

fn sides_in_line(mut lines: Vec<i64>) -> usize {
    lines.sort();
    let mut sides = 1;
    if lines.len() > 1 {
        for i in 1..lines.len() {
            if lines[i] - lines[i - 1] > 1 {
                sides += 1;
            }
        }
    }
    sides
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = 0;
        for (x, y, _) in &self.plots {
            if !self.plots.contains(&(x - 1, *y, self.plot_type)) {
                perimeter += 1;
            }
            if !self.plots.contains(&(x + 1, *y, self.plot_type)) {
                perimeter += 1;
            }
            if !self.plots.contains(&(*x, y - 1, self.plot_type)) {
                perimeter += 1;
            }
            if !self.plots.contains(&(*x, y + 1, self.plot_type)) {
                perimeter += 1;
            }
        }
        perimeter
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn number_of_sides(&self) -> usize {
        let c = self.plot_type;
        let mut top_plots: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
        let mut bottom_plots: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
        let mut left_plots: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
        let mut right_plots: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
        self.plots.iter().for_each(|(x, y, _)| {
            if !self.plots.contains(&(*x, y - 1, c)) {
                // Top side
                top_plots.entry(*y).or_default().push(*x);
            }
            if !self.plots.contains(&(*x, y + 1, c)) {
                // Bottom side
                bottom_plots.entry(*y).or_default().push(*x);
            }
            if !self.plots.contains(&(x - 1, *y, c)) {
                // Left side
                left_plots.entry(*x).or_default().push(*y);
            }
            if !self.plots.contains(&(x + 1, *y, c)) {
                // Right side
                right_plots.entry(*x).or_default().push(*y);
            }
        });
        let mut sides = 0;
        top_plots.into_values().for_each(|lines| {
            sides += sides_in_line(lines);
        });
        bottom_plots.into_values().for_each(|lines| {
            sides += sides_in_line(lines);
        });
        left_plots.into_values().for_each(|lines| {
            sides += sides_in_line(lines);
        });
        right_plots.into_values().for_each(|lines| {
            sides += sides_in_line(lines);
        });
        sides
    }

    fn discounted_price(&self) -> usize {
        self.area() * self.number_of_sides()
    }
}

fn fill_region_rec((x, y, c): (i64, i64, char), free_plots: &mut PlotsSet, region: &mut Region) {
    assert_eq!(c, region.plot_type);
    if free_plots.contains(&(x, y, c)) {
        free_plots.remove(&(x, y, c));
        region.plots.insert((x, y, c));
        fill_region_rec((x - 1, y, c), free_plots, region);
        fill_region_rec((x + 1, y, c), free_plots, region);
        fill_region_rec((x, y - 1, c), free_plots, region);
        fill_region_rec((x, y + 1, c), free_plots, region);
    }
}

fn build_regions(grid: &Vec<Vec<char>>) -> Vec<Region> {
    let mut free_plots = PlotsSet::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            free_plots.insert((x as i64, y as i64, *c));
        }
    }
    let mut regions = Vec::new();
    while !free_plots.is_empty() {
        let (x, y, c) = free_plots.iter().next().unwrap().clone();
        let mut region = Region {
            plot_type: c,
            plots: PlotsSet::new(),
        };
        fill_region_rec((x, y, c), &mut free_plots, &mut region);
        regions.push(region);
    }
    regions
}

fn main() {
    let filename = "inputs/day12.txt";
    let grid = read_file_to_grid(filename).expect("Failed to read file");
    let regions = build_regions(&grid);

    // Part 1
    let total_price = regions.iter().map(|r| r.price()).sum::<usize>();
    assert_eq!(total_price, 1371306);
    println!("Total price: {}", total_price);

    // Part 2
    let total_discounted_price = regions.iter().map(|r| r.discounted_price()).sum::<usize>();
    assert_eq!(total_discounted_price, 805880);
    println!("Total discounted price: {}", total_discounted_price);
}
