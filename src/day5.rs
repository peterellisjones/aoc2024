use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use nom::{
    Finish,
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};

use crate::Day;

pub struct Day5;

impl Day for Day5 {
    const DAY_NUMBER: i64 = 5;
    const PART1_EXAMPLE_SOLUTION: i64 = 143;
    const PART2_EXAMPLE_SOLUTION: i64 = 123;

    fn part1(raw_input: &str) -> i64 {
        let (ordering_pairs, update_pages) = parse(raw_input).unwrap();

        let orderings = Orderings::new(&ordering_pairs);
        let mut sum = 0;

        for pages in update_pages.iter() {
            if pages.is_sorted_by(|left, right| orderings.compare(left, right) != Ordering::Greater)
            {
                sum += pages[pages.len() / 2]
            }
        }

        sum
    }

    fn part2(raw_input: &str) -> i64 {
        let (ordering_pairs, update_pages) = parse(raw_input).unwrap();

        let orderings = Orderings::new(&ordering_pairs);
        let mut sum = 0;

        for pages in update_pages.iter() {
            let mut pages_ordered = pages.clone();

            pages_ordered.sort_by(|left, right| orderings.compare(left, right));

            if !pages.iter().eq(pages_ordered.iter()) {
                sum += pages_ordered[pages_ordered.len() / 2]
            }
        }

        sum
    }
}

struct Orderings(HashMap<i64, HashSet<i64>>);

impl Orderings {
    fn new(pairs: &Vec<(i64, i64)>) -> Orderings {
        let mut hash_map = HashMap::new();

        for (left, right) in pairs {
            hash_map
                .entry(*left)
                .or_insert_with(|| HashSet::new())
                .insert(*right);
        }

        Orderings(hash_map)
    }

    fn compare(self: &Self, left: &i64, right: &i64) -> Ordering {
        if Some(true)
            == self
                .0
                .get(left)
                .and_then(|values| Some(values.contains(right)))
        {
            return Ordering::Less;
        } else if Some(true)
            == self
                .0
                .get(right)
                .and_then(|values| Some(values.contains(left)))
        {
            return Ordering::Greater;
        }

        return Ordering::Equal;
    }
}

pub fn parse(input: &str) -> Result<(Vec<(i64, i64)>, Vec<Vec<i64>>), nom::error::Error<&str>> {
    let page_ordering_pairs = many1(terminated(
        separated_pair(nom_i64, tag("|"), nom_i64),
        line_ending,
    ));

    let update_pages = many1(terminated(separated_list1(tag(","), nom_i64), line_ending));

    separated_pair(page_ordering_pairs, line_ending, update_pages)(input)
        .finish()
        .map(|(_, (pairs, pages))| (pairs, pages))
}
