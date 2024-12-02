use std::collections::HashMap;

use itertools::{FoldWhile, Itertools};

use crate::{
    Day,
    utils::{parse_integer_list, parse_integer_pairs},
};

pub struct Day2;

#[derive(PartialEq, Debug)]
enum Dir {
    Start,
    Unknown,
    Decreasing,
    Increasing,
}

impl Day for Day2 {
    const DAY_NUMBER: i64 = 2;
    const PART1_EXAMPLE_SOLUTION: i64 = 2;
    const PART2_EXAMPLE_SOLUTION: i64 = 4;

    fn part1(raw_input: &str) -> i64 {
        let input = parse_integer_list(raw_input).unwrap();

        let num_safe = input
            .iter()
            .filter(|&levels| -> bool { is_safe(levels, None) })
            .count();

        num_safe as i64
    }

    fn part2(raw_input: &str) -> i64 {
        let input = parse_integer_list(raw_input).unwrap();

        let num_safe = input
            .iter()
            .filter(|&levels| -> bool {
                if is_safe(levels, None) {
                    true
                } else {
                    for i in 0..levels.len() {
                        if is_safe(levels, Some(i)) {
                            return true;
                        }
                    }
                    false
                }
            })
            .count();

        num_safe as i64
    }
}

fn is_safe(levels: &Vec<i64>, skip_index: Option<usize>) -> bool {
    let result = levels.iter().enumerate().fold_while(
        (Dir::Start, 0i64),
        |(prev_dir, prev_val), (idx, &level)| -> FoldWhile<(Dir, i64)> {
            if skip_index == Some(idx) {
                return FoldWhile::Continue((prev_dir, prev_val));
            }

            let diff = (prev_val - level).abs();
            let diff_is_safe = diff == 1 || diff == 2 || diff == 3;
            let is_increasing = level > prev_val;

            match (prev_dir, diff_is_safe) {
                (Dir::Start, _) => FoldWhile::Continue((Dir::Unknown, level)),
                (_, false) => FoldWhile::Done((Dir::Increasing, level)),
                (Dir::Unknown, true) => {
                    if is_increasing {
                        FoldWhile::Continue((Dir::Increasing, level))
                    } else {
                        FoldWhile::Continue((Dir::Decreasing, level))
                    }
                }
                (Dir::Increasing, true) => {
                    if is_increasing {
                        FoldWhile::Continue((Dir::Increasing, level))
                    } else {
                        FoldWhile::Done((Dir::Increasing, level))
                    }
                }
                (Dir::Decreasing, true) => {
                    if !is_increasing {
                        FoldWhile::Continue((Dir::Decreasing, level))
                    } else {
                        FoldWhile::Done((Dir::Increasing, level))
                    }
                }
            }
        },
    );

    match result {
        FoldWhile::Continue(_) => true,
        FoldWhile::Done(_) => false,
    }
}
