use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

use crate::{Day, utils::parse_digit_grid};

pub struct Day10;

impl Day for Day10 {
    const DAY_NUMBER: i64 = 10;
    const PART1_EXAMPLE_SOLUTION: i64 = 36;
    const PART2_EXAMPLE_SOLUTION: i64 = 81;

    fn part1(raw_input: &str) -> i64 {
        trailhead_rating_score(&raw_input, false)
    }

    fn part2(raw_input: &str) -> i64 {
        trailhead_rating_score(&raw_input, true)
    }
}

fn trailhead_rating_score(raw_input: &str, allow_multiple_routes: bool) -> i64 {
    let input = parse_digit_grid(raw_input).unwrap();

    let length = input.len();
    let width = input[0].len();

    let trailheads: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .map(|(y, row)| row.iter().enumerate().map(move |(x, h)| (y, x, *h)))
        .flatten()
        .filter(|(_, _, h)| *h == 0)
        .map(|(y, x, _)| (y, x))
        .collect();

    trailheads
        .into_par_iter()
        .map(|(trailhead_y, trailhead_x)| {
            let mut visited: HashSet<(usize, usize)> = HashSet::new();

            // stack keeps track of next squares to explore
            let mut stack: Vec<(usize, usize)> = vec![(trailhead_y, trailhead_x)];

            // trail rating for this trailhead.
            // if allow_multiple_routes=false then this is just the number of reachable peaks
            let mut trail_ratings = 0;

            while !stack.is_empty() {
                let (y, x) = stack.pop().unwrap();

                visited.insert((y, x));

                let next_height = input[y][x] + 1;
                let mut neighbours = Vec::new();

                if next_height == 10 {
                    trail_ratings += 1;
                } else {
                    if y > 0
                        && input[y - 1][x] == next_height
                        && (allow_multiple_routes || !visited.contains(&(y - 1, x)))
                    {
                        neighbours.push((y - 1, x));
                    }
                    if y < length - 1
                        && input[y + 1][x] == next_height
                        && (allow_multiple_routes || !visited.contains(&(y + 1, x)))
                    {
                        neighbours.push((y + 1, x));
                    }
                    if x > 0
                        && input[y][x - 1] == next_height
                        && (allow_multiple_routes || !visited.contains(&(y, x - 1)))
                    {
                        neighbours.push((y, x - 1));
                    }
                    if x < width - 1
                        && input[y][x + 1] == next_height
                        && (allow_multiple_routes || !visited.contains(&(y, x + 1)))
                    {
                        neighbours.push((y, x + 1));
                    }

                    stack.extend(neighbours);
                }
            }

            trail_ratings
        })
        .sum()
}
