use std::collections::HashMap;

use crate::{Day, utils::parse_integer_pairs};

pub struct Day1;

impl Day for Day1 {
    const DAY_NUMBER: i64 = 1;
    const PART1_EXAMPLE_SOLUTION: i64 = 11;
    const PART2_EXAMPLE_SOLUTION: i64 = 31;

    fn part1(raw_input: &str) -> i64 {
        let input = parse_integer_pairs(raw_input).unwrap();

        let mut left = input.iter().map(|f| f.0).collect::<Vec<i64>>();
        let mut right = input.iter().map(|f| f.1).collect::<Vec<i64>>();

        left.sort();
        right.sort();

        debug_assert_eq!(left.len(), right.len());

        left.iter()
            .zip(right.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0, |acc, x| acc + x)
    }

    fn part2(raw_input: &str) -> i64 {
        let input = parse_integer_pairs(raw_input).unwrap();

        let mut right_list_frequencies: HashMap<i64, i64> = HashMap::new();

        for x in input.iter().map(|f| f.1) {
            *right_list_frequencies.entry(x).or_insert_with(|| 0i64) += 1;
        }

        input.iter().map(|f| f.0).fold(0, |acc, x| {
            acc + x * right_list_frequencies.get(&x).or(Some(&0i64)).unwrap()
        })
    }
}
