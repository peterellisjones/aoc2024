use std::collections::BTreeMap;

use crate::Day;

pub struct Day9;

impl Day for Day9 {
    const DAY_NUMBER: i64 = 9;
    const PART1_EXAMPLE_SOLUTION: i64 = 1928;
    const PART2_EXAMPLE_SOLUTION: i64 = 2858;

    fn part1(raw_input: &str) -> i64 {
        let mut blocks = parse_blocks(raw_input);

        // compaction with fragmentation
        let mut block_id = 0usize;
        let mut checksum = 0;

        while block_id < blocks.len() {
            if let Some(file_id) = blocks[block_id] {
                checksum += file_id * (block_id as i64);
                block_id += 1;
            } else if let Some(file_id) = blocks.pop().unwrap() {
                blocks[block_id] = Some(file_id);
            }
        }

        checksum
    }

    fn part2(raw_input: &str) -> i64 {
        // files =>  block_id, size
        // free spaces => block_id, size
        let (files, free_spaces) = parse_files_and_free_spaces(raw_input);

        // maps free space starting at block X to free space size
        let mut free_spaces_by_block_id: BTreeMap<i64, i64> = BTreeMap::from_iter(free_spaces);

        let mut checksum = 0;

        // compaction without fragmentation
        for (file_id, &(file_block_id, file_size)) in files.iter().enumerate().rev() {
            // are there any free blocks before this file that can fit it?
            let free_space = free_spaces_by_block_id
                .iter()
                .take_while(|&(&block_id, _)| block_id < file_block_id)
                .find(|&(_, &size)| size >= file_size);

            let mut new_file_block_id = file_block_id;

            // If we found a free space...
            if let Some((&free_space_block_id, &free_space_size)) = free_space {
                // 1. store the file in the free space
                new_file_block_id = free_space_block_id;

                // 2. remove the free space we've used
                free_spaces_by_block_id.remove(&free_space_block_id);

                // 3. add any remaining space behind the file back into the free spaces btree
                if free_space_size > file_size {
                    free_spaces_by_block_id
                        .insert(free_space_block_id + file_size, free_space_size - file_size);
                }
            }

            // use arithmetic series formula for checksum
            checksum += (file_id as i64) * (new_file_block_id * 2 + file_size - 1) * file_size / 2;
        }

        // checksum
        checksum
    }
}

fn parse_files_and_free_spaces(raw_input: &str) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    let mut file_layout = Vec::new();
    let mut free_space_layout = Vec::new();

    let mut block_id = 0i64;
    for (idx, c) in raw_input.chars().enumerate() {
        if let Some(size) = c.to_digit(10) {
            if size > 0 {
                let size = size as i64;
                if idx % 2 == 0 {
                    file_layout.push((block_id, size));
                } else {
                    free_space_layout.push((block_id, size));
                }

                block_id += size;
            }
        } else {
            break;
        }
    }

    (file_layout, free_space_layout)
}

fn parse_blocks(raw_input: &str) -> Vec<Option<i64>> {
    let mut blocks = Vec::new();

    let mut file_id = 0i64;
    for (idx, c) in raw_input.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            let block = if idx % 2 == 0 {
                let b = Some(file_id);
                file_id += 1;
                b
            } else {
                None
            };

            for _ in 0..digit {
                blocks.push(block);
            }
        } else {
            break;
        }
    }

    blocks
}
