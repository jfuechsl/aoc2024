use aoc2024::utils::file::load_file_lines;

fn group_blocks(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut blocks = Vec::new();
    let mut current_block = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            if !current_block.is_empty() {
                blocks.push(current_block);
                current_block = Vec::new();
            }
        } else {
            current_block.push(line);
        }
    }

    if !current_block.is_empty() {
        blocks.push(current_block);
    }

    blocks
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SchematicKind {
    Lock,
    Key,
}

#[derive(Debug, Clone)]
struct Schematic {
    kind: SchematicKind,
    heights: Vec<usize>,
    max_height: usize,
}

impl Schematic {
    fn from_line_blocks(lines: &Vec<String>) -> Self {
        let num_columns = lines[0].len();
        let mut heights = vec![0; num_columns];
        // Lock schematic: Top-down filled
        // Key schematic: Bottom-up filled
        let kind = if lines[0].chars().all(|c| c == '#') {
            SchematicKind::Lock
        } else {
            SchematicKind::Key
        };
        for line in lines.iter().skip(1) {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    heights[j] += 1;
                }
            }
        }
        let max_height = lines.len() - 1;
        Self {
            kind,
            heights,
            max_height,
        }
    }

    fn fits_with(&self, other: &Self) -> bool {
        assert_eq!(self.heights.len(), other.heights.len());
        assert_eq!(self.max_height, other.max_height);
        assert_ne!(self.kind, other.kind);
        let max_height = self.max_height;
        self.heights
            .iter()
            .zip(other.heights.iter())
            .all(|(h1, h2)| h1 + h2 <= max_height)
    }
}

fn main() {
    let filename = "inputs/day25.txt";
    let lines = load_file_lines(filename).expect("Invalid filename");
    let blocks = group_blocks(lines);
    let (locks, keys): (Vec<_>, Vec<_>) = blocks
        .iter()
        .map(|block| Schematic::from_line_blocks(block))
        .partition(|schematic| schematic.kind == SchematicKind::Lock);

    let mut num_fits = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if lock.fits_with(key) {
                num_fits += 1;
            }
        }
    }
    assert_eq!(num_fits, 3508);
    println!("Number of fits: {}", num_fits);
}
