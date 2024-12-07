use rayon::prelude::*;
use std::collections::HashMap;

use crate::{Day, utils::parse_char_grid};
pub struct Day6;

impl Day for Day6 {
    const DAY_NUMBER: i64 = 6;
    const PART1_EXAMPLE_SOLUTION: i64 = 41;
    const PART2_EXAMPLE_SOLUTION: i64 = 6;

    fn part1(raw_input: &str) -> i64 {
        let map = parse_char_grid(raw_input).unwrap();
        let start_position = find_guard_start(&map);
        let start_direction: i64 = 0;
        let mut route: HashMap<(i64, i64), i64> = HashMap::new();

        let (found_loop, _) = walk_guard(start_position, start_direction, &map, &mut route);

        assert!(!found_loop);

        route.len() as i64
    }

    fn part2(raw_input: &str) -> i64 {
        let map = parse_char_grid(raw_input).unwrap();

        let start_position = find_guard_start(&map);
        let start_direction: i64 = 0;
        let mut route_without_obstacles: HashMap<(i64, i64), i64> = HashMap::new();

        let (found_loop, path) = walk_guard(
            start_position,
            start_direction,
            &map,
            &mut route_without_obstacles,
        );

        assert!(!found_loop);

        (0..(path.len() - 1))
            .into_par_iter()
            .map(|i| {
                let (start_position, start_direction) = path[i];
                let (next_position, _) = path[i + 1];

                let mut route = HashMap::new();

                route.insert(start_position, start_direction);

                let mut map = map.clone();
                map[next_position.0 as usize][next_position.1 as usize] = 'O';

                let (found_loop, _) = walk_guard(start_position, start_direction, &map, &mut route);

                if found_loop { 1 } else { 0 }
            })
            .sum()
    }
}

fn find_guard_start(map: &Vec<Vec<char>>) -> (i64, i64) {
    for (row, row_vec) in map.iter().enumerate() {
        for (col, &elem) in row_vec.iter().enumerate() {
            if elem == '^' {
                return (row as i64, col as i64);
            }
        }
    }

    unreachable!();
}

fn walk_guard(
    start_position: (i64, i64),
    start_direction: i64,
    map: &Vec<Vec<char>>,
    route: &mut HashMap<(i64, i64), i64>,
) -> (bool, Vec<((i64, i64), i64)>) {
    let mut guard = start_position;
    let mut guard_direction: i64 = start_direction;

    let height = map.len() as i64;
    let width = map[0].len() as i64;

    let mut path = Vec::new();
    path.push((guard, guard_direction));
    route.insert(guard, guard_direction);

    loop {
        let new_guard_position = match guard_direction {
            0 => (guard.0 - 1, guard.1), // up
            1 => (guard.0, guard.1 + 1), // right
            2 => (guard.0 + 1, guard.1), // down
            3 => (guard.0, guard.1 - 1), // left
            _ => unreachable!(),
        };

        // check bounds
        if new_guard_position.0 < 0
            || new_guard_position.0 >= height
            || new_guard_position.1 < 0
            || new_guard_position.1 >= width
        {
            break;
        }

        // check loop
        // if we arrive at the same point going the same direction then there is a loop
        if let Some(&previous_guard_direction_at_position) = route.get(&new_guard_position) {
            if previous_guard_direction_at_position == guard_direction {
                return (true, path);
            }
        }

        match map[new_guard_position.0 as usize][new_guard_position.1 as usize] {
            '.' | '^' => {
                // space is free so guard can move into this position
                guard = new_guard_position;
                route.insert(guard, guard_direction);
                path.push((guard, guard_direction));
            }
            '#' | 'O' => {
                // space is not free so we need to rotate 90* to the right
                guard_direction = (guard_direction + 1) % 4;
            }
            _ => unreachable!(),
        }
    }

    (false, path)
}
