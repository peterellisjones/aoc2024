use std::collections::HashMap;

use fnv::{FnvHashMap, FnvHasher};
use nom::{
    Finish,
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, line_ending, multispace1},
    error::Error as NomError,
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
};

use crate::Day;

pub struct Day14;

impl Day for Day14 {
    const DAY_NUMBER: i64 = 14;
    const PART1_EXAMPLE_SOLUTION: i64 = 12;
    const PART2_EXAMPLE_SOLUTION: i64 = 12;

    fn part1(raw_input: &str) -> i64 {
        let mut robots = parse(raw_input).unwrap();

        let (dim_x, dim_y) = if robots.len() > 100 {
            (101, 103)
        } else {
            (11, 7)
        };

        let mut quandrant_counts: FnvHashMap<i64, i64> = FnvHashMap::default();

        for _ in 0..100 {
            robots.iter_mut().for_each(|r| r.tick(dim_x, dim_y));
        }

        for r in robots.iter() {
            if let Some(q) = r.quadrant(dim_x, dim_y) {
                *quandrant_counts.entry(q).or_default() += 1;
            }
        }

        let safety_factor = quandrant_counts
            .iter()
            .fold(1i64, |acc, (_, count)| acc * count);

        safety_factor
    }

    fn part2(raw_input: &str) -> i64 {
        let mut robots = parse(raw_input).unwrap();

        let (dim_x, dim_y) = if robots.len() > 100 {
            (101, 103)
        } else {
            (11, 7)
        };

        'tick_loop: for i in 0..10000 {
            robots.iter_mut().for_each(|r| r.tick(dim_x, dim_y));

            let mut row_char_counts: FnvHashMap<i64, usize> = FnvHashMap::default();

            for r in robots.iter() {
                *row_char_counts.entry(r.pos_y).or_default() += 1;
            }

            if i != 6751 {
                continue;
            }

            let mut grid = vec![vec![0i64; dim_x as usize]; dim_y as usize];

            for r in robots.iter() {
                grid[r.pos_y as usize][r.pos_x as usize] += 1;
            }

            // christmas tree image has horizontal row of 30 robots starting at y=37, x=45
            for i in 45..75 {
                if grid[37][i] != 1 {
                    continue 'tick_loop;
                }
            }

            return i + 1;
        }

        unreachable!()
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos_x: i64,
    pos_y: i64,
    v_x: i64,
    v_y: i64,
}

impl Robot {
    fn tick(&mut self, dim_x: i64, dim_y: i64) {
        self.pos_x = (self.pos_x + self.v_x + dim_x) % dim_x;
        self.pos_y = (self.pos_y + self.v_y + dim_y) % dim_y;
    }

    fn quadrant(&self, dim_x: i64, dim_y: i64) -> Option<i64> {
        debug_assert!(dim_x % 2 != 0);
        debug_assert!(dim_y % 2 != 0);

        if self.pos_x < dim_x / 2 {
            if self.pos_y < dim_y / 2 {
                Some(1i64)
            } else if self.pos_y > dim_y / 2 {
                Some(2i64)
            } else {
                None
            }
        } else if self.pos_x > dim_x / 2 {
            if self.pos_y < dim_y / 2 {
                Some(3i64)
            } else if self.pos_y > dim_y / 2 {
                Some(4i64)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Result<Vec<Robot>, NomError<&str>> {
    // p=0,4 v=3,-3
    // p=6,3 v=-1,-3
    // p=10,3 v=-1,2

    let robot = terminated(
        separated_pair(
            preceded(tag("p="), separated_pair(nom_i64, tag(","), nom_i64)),
            multispace1,
            preceded(tag("v="), separated_pair(nom_i64, tag(","), nom_i64)),
        ),
        line_ending,
    );

    many1(robot)(input).finish().map(|(_, x)| {
        x.iter()
            .map(|((px, py), (vx, vy))| Robot {
                pos_x: *px,
                pos_y: *py,
                v_x: *vx,
                v_y: *vy,
            })
            .collect()
    })
}
