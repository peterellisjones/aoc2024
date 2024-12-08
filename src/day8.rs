use std::collections::{HashMap, HashSet};

use crate::{Day, utils::parse_char_grid};
use itertools::Itertools;

pub struct Day8;

impl Day for Day8 {
    const DAY_NUMBER: i64 = 8;
    const PART1_EXAMPLE_SOLUTION: i64 = 14;
    const PART2_EXAMPLE_SOLUTION: i64 = 34;

    fn part1(raw_input: &str) -> i64 {
        unique_antinode_count(raw_input, false)
    }

    fn part2(raw_input: &str) -> i64 {
        unique_antinode_count(raw_input, true)
    }
}

fn unique_antinode_count(raw_input: &str, allow_echos: bool) -> i64 {
    let input = parse_char_grid(raw_input).unwrap();
    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let mut antinode_locations: HashSet<(i64, i64)> = HashSet::new();

    let height_range = 0..input.len() as i64;
    let width_range = 0..input[0].len() as i64;

    // collect antennas
    for (col, row_vec) in input.iter().enumerate() {
        for (row, antenna_code) in row_vec.iter().enumerate() {
            if *antenna_code != '.' {
                antennas
                    .entry(*antenna_code)
                    .or_insert_with(|| Vec::new())
                    .push((col as i64, row as i64));
            }
        }
    }

    // calculate unique antinodes
    for (_, locations) in antennas.iter() {
        for (left, right) in locations.iter().tuple_combinations() {
            debug_assert_ne!(left, right);

            if allow_echos {
                antinode_locations.insert(*left);
                antinode_locations.insert(*right);
            }

            let mut antenna1_col = left.0 + (left.0 - right.0);
            let mut antenna1_row = left.1 + (left.1 - right.1);

            while height_range.contains(&antenna1_col) && width_range.contains(&antenna1_row) {
                antinode_locations.insert((antenna1_col, antenna1_row));

                antenna1_col += left.0 - right.0;
                antenna1_row += left.1 - right.1;

                if !allow_echos {
                    break;
                }
            }

            let mut antenna2_col = right.0 - (left.0 - right.0);
            let mut antenna2_row = right.1 - (left.1 - right.1);

            while height_range.contains(&antenna2_col) && width_range.contains(&antenna2_row) {
                antinode_locations.insert((antenna2_col, antenna2_row));

                antenna2_col -= left.0 - right.0;
                antenna2_row -= left.1 - right.1;

                if !allow_echos {
                    break;
                }
            }
        }
    }

    antinode_locations.len() as i64
}
