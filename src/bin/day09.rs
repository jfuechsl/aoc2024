use std::{
    cmp::{max, min},
    collections::BTreeMap,
    vec,
};

use aoc2024::utils::file::read_lines;

struct Disk {
    blocks: Vec<Option<i64>>,
}

impl Disk {
    fn from_layout_string(layout_string: &String) -> Self {
        let mut file_id = 0;
        let mut cur_is_file = true;
        let mut blocks = Vec::new();
        for char in layout_string.trim().chars() {
            let num = char
                .to_digit(10)
                .expect(format!("Invalid digit: {}", char).as_str());
            if cur_is_file {
                blocks.extend(vec![Some(file_id); num as usize]);
                file_id += 1;
                cur_is_file = false;
            } else {
                blocks.extend(vec![None; num as usize]);
                cur_is_file = true;
            }
        }
        Self { blocks }
    }

    fn compact_blocks(&mut self) {
        let mut free_ptr = self
            .blocks
            .iter()
            .enumerate()
            .find_map(|(i, x)| match x {
                None => Some(i),
                _ => None,
            })
            .expect("No free space found");
        let mut cur_block = self.blocks.len() - 1;
        while cur_block > free_ptr {
            if self.blocks[cur_block].is_some() {
                assert!(self.blocks[free_ptr].is_none());
                self.blocks.swap(cur_block, free_ptr);
                loop {
                    free_ptr += 1;
                    if self.blocks[free_ptr].is_none() {
                        break;
                    }
                }
            }
            cur_block -= 1;
        }
    }

    fn swap_block_ranges(&mut self, blk1_start: usize, blk2_start: usize, blk_len: usize) {
        let blk1_end = blk1_start + blk_len;
        let blk2_end = blk2_start + blk_len;
        assert!(blk1_end <= self.blocks.len());
        assert!(blk2_end <= self.blocks.len());
        assert!(min(blk1_end, blk2_end) <= max(blk1_start, blk2_start));
        for i in 0..blk_len {
            self.blocks.swap(blk1_start + i, blk2_start + i);
        }
    }

    fn iter_empty(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.blocks
            .iter()
            .enumerate()
            .scan(None, |state, (i, block)| match (&state, block) {
                (None, None) => {
                    *state = Some(i);
                    Some(None)
                }
                (Some(start), Some(_)) => {
                    let range = (*start, i);
                    *state = None;
                    Some(Some(range))
                }
                _ => Some(None),
            })
            .filter_map(|x| x)
    }

    fn iter_files(&self) -> FilesIter {
        FilesIter {
            disk: self,
            ptr: None,
            done: false,
        }
    }

    #[allow(dead_code)]
    fn compact_files(&mut self) {
        let mut cur_file_id = self.iter_files().rev().next().unwrap().2;
        loop {
            let mut files_iter = self
                .iter_files()
                .rev()
                .skip_while(|(_, _, f_id)| *f_id > cur_file_id);
            let swp_res = files_iter.find_map(|(f_start, f_end, f_id)| {
                let f_size = f_end - f_start;
                let mut empty_iter = self.iter_empty();
                let e_res = empty_iter.find(|(e_start, e_end)| {
                    let e_size = e_end - e_start;
                    e_size >= f_size && *e_start < f_start
                });
                match e_res {
                    Some((e_start, _)) => Some((e_start, f_start, f_size, f_id)),
                    None => None,
                }
            });
            match swp_res {
                Some((e_start, f_start, f_size, f_id)) => {
                    self.swap_block_ranges(e_start, f_start, f_size);
                    if f_id == 0 {
                        break;
                    }
                    cur_file_id = f_id - 1;
                }
                None => {
                    cur_file_id -= 1;
                }
            }
            if cur_file_id == 0 {
                break;
            }
        }
    }

    fn compact_files2(&mut self) {
        let mut free_blocks: BTreeMap<usize, BTreeMap<usize, usize>> = BTreeMap::new();
        for (start, end) in self.iter_empty() {
            let len = end - start;
            for s in 1..=len {
                free_blocks
                    .entry(s)
                    .or_insert_with(BTreeMap::new)
                    .insert(start, len);
            }
        }
        let file_positions = self.iter_files().rev().collect::<Vec<_>>();
        for (fstart, fend, _fid) in file_positions {
            let flen = fend - fstart;
            assert!(flen > 0 && flen <= 9);
            let remove_empty = match free_blocks.get(&flen) {
                Some(free_blocks) => match free_blocks.first_key_value() {
                    Some((estart, elen)) => {
                        if fstart > *estart {
                            self.swap_block_ranges(*estart, fstart, flen);
                            Some((*estart, *elen, flen, fstart))
                        } else {
                            None
                        }
                    }
                    None => None,
                },
                None => None,
            };
            if let Some((estart, elen, flen, fstart)) = remove_empty {
                let new_estart = estart + flen;
                let new_elen = elen - flen;
                // Remove the old empty block
                for s in 1..=elen {
                    free_blocks
                        .entry(s)
                        .or_insert_with(BTreeMap::new)
                        .remove(&estart);
                }
                // Add the new empty block
                for s in 1..=new_elen {
                    free_blocks
                        .entry(s)
                        .or_insert_with(BTreeMap::new)
                        .insert(new_estart, new_elen);
                }
                // Add new empty blocks where the file used to be
                // Get the potential empty block after (to be merged)
                let next_empty_start = fstart + flen;
                let next_empty_len = *free_blocks
                    .get(&1)
                    .and_then(|x| x.get(&next_empty_start))
                    .unwrap_or(&0);
                let new_empty_len = flen + next_empty_len;
                for s in 1..=new_empty_len {
                    free_blocks
                        .entry(s)
                        .or_insert_with(BTreeMap::new)
                        .insert(fstart, new_empty_len);
                }
            }
        }
    }

    fn checksum(&self) -> i64 {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(i, x)| match x {
                Some(x) => Some((i, x)),
                None => None,
            })
            .map(|(i, x)| i as i64 * x)
            .sum()
    }
}

struct FilesIter<'a> {
    disk: &'a Disk,
    ptr: Option<usize>,
    done: bool,
}

impl Iterator for FilesIter<'_> {
    type Item = (usize, usize, i64);

    fn next(&mut self) -> Option<(usize, usize, i64)> {
        if self.done {
            return None;
        }
        let mut ptr = self.ptr.unwrap_or(0);
        while ptr < self.disk.blocks.len() {
            while self.disk.blocks[ptr].is_none() {
                ptr += 1;
            }
            let file_id = self.disk.blocks[ptr].expect("Invalid file ID");
            let mut end = ptr + 1;
            while end < self.disk.blocks.len() {
                match self.disk.blocks.get(end) {
                    Some(Some(id)) if *id == file_id => end += 1,
                    _ => break,
                }
            }
            let start = ptr;
            self.ptr = Some(end);
            return Some((start, end, file_id));
        }
        self.done = true;
        None
    }
}

impl DoubleEndedIterator for FilesIter<'_> {
    fn next_back(&mut self) -> Option<(usize, usize, i64)> {
        if self.done {
            return None;
        }
        let mut ptr = self.ptr.unwrap_or(self.disk.blocks.len() - 1);
        loop {
            while self.disk.blocks[ptr].is_none() {
                assert!(ptr > 0);
                ptr -= 1;
            }
            assert!(self.disk.blocks[ptr].is_some());
            let file_id = self.disk.blocks[ptr].expect("Invalid file ID");
            let end = ptr + 1;
            while ptr > 0 {
                if ptr == 0 {
                    break;
                }
                match self.disk.blocks[ptr - 1] {
                    Some(id) if id == file_id => ptr -= 1,
                    _ => break,
                }
            }
            let start = ptr;
            if ptr == 0 {
                self.done = true;
            } else {
                ptr -= 1;
            }
            self.ptr = Some(ptr);
            return Some((start, end, file_id));
        }
    }
}

fn main() {
    let filename = "inputs/day09.txt";
    let disk_layout = read_lines(filename).expect("Failed to read file");

    // Part 1
    let mut disk = Disk::from_layout_string(&disk_layout);
    disk.compact_blocks();
    let checksum = disk.checksum();
    assert_eq!(checksum, 6607511583593);
    println!("Checksum (Block compaction): {}", checksum);

    // Part 2
    let mut disk = Disk::from_layout_string(&disk_layout);
    // disk.compact_files();
    disk.compact_files2();
    let checksum = disk.checksum();
    assert_eq!(checksum, 6636608781232);
    println!("Checksum (File compaction): {}", checksum);
}
