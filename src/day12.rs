use fnv::{FnvHashMap, FnvHashSet};

use crate::{Day, utils::*};

pub struct Day12;

impl Day for Day12 {
    const DAY_NUMBER: i64 = 12;
    const PART1_EXAMPLE_SOLUTION: i64 = 1930;
    const PART2_EXAMPLE_SOLUTION: i64 = 65601038650482;

    fn part1(raw_input: &str) -> i64 {
        todo!();

        let chars = parse_char_grid(raw_input).unwrap();
        let regions = convert_to_regions(&chars);

        let mut areas: FnvHashMap<i64, i64> = FnvHashMap::default();
        let mut perimeters: FnvHashMap<i64, i64> = FnvHashMap::default();
        let mut region_names: FnvHashMap<i64, char> = FnvHashMap::default();

        for (y, row) in regions.iter().enumerate() {
            for (x, r) in row.iter().enumerate() {
                region_names.insert(*r, chars[y][x]);

                *areas.entry(*r).or_insert(0) += 1;

                let mut perimeter = 0;
                if x == 0 {
                    perimeter += 1;
                } else if regions[y][x - 1] != *r {
                    perimeter += 1;
                }

                if y == 0 {
                    perimeter += 1;
                } else if regions[y - 1][x] != *r {
                    perimeter += 1;
                }

                if x == row.len() - 1 {
                    perimeter += 1;
                } else if regions[y][x + 1] != *r {
                    perimeter += 1;
                }

                if y == regions.len() - 1 {
                    perimeter += 1;
                } else if regions[y + 1][x] != *r {
                    perimeter += 1;
                }

                *perimeters.entry(*r).or_insert(0) += perimeter;
            }
        }

        // for i in 1..areas.len() + 1 {
        //     let r = &(i as i64);
        //     println!(
        //         "region: {} ({}), area: {}, perimeter: {}, price: {}",
        //         region_names[r],
        //         r,
        //         areas[r],
        //         perimeters[r],
        //         areas[r] * perimeters[r]
        //     );
        // }

        let result: i64 = areas.iter().map(|(r, area)| area * perimeters[r]).sum();

        result
    }

    fn part2(raw_input: &str) -> i64 {
        let chars = parse_char_grid(raw_input).unwrap();
        let regions = convert_to_regions(&chars);
        let mut areas: FnvHashMap<i64, i64> = FnvHashMap::default();
        let mut side_counts: FnvHashMap<i64, i64> = FnvHashMap::default();
        let mut region_names: FnvHashMap<i64, char> = FnvHashMap::default();

        for (y, row) in regions.iter().enumerate() {
            for (x, r) in row.iter().enumerate() {
                region_names.insert(*r, chars[y][x]);
                *areas.entry(*r).or_insert(0) += 1;
            }
        }

        for (y, row) in regions.iter().enumerate() {
            for (x, r) in row.iter().enumerate() {
                if side_counts.contains_key(r) {
                    continue;
                }

                let mut side_count = 1;
                let mut direction = 0i64;

                let mut y1 = y;
                let mut x1 = x + 1;

                let mut count = 0;

                while y1 != y || x1 != x {
                    count += 1;
                    if count > 20 {
                        break;
                    }

                    println!("{}, {}, {}", x1, y1, direction);

                    if direction == 0 {
                        // going right
                        if x1 >= row.len() - 1 || regions[y1][x1] != *r {
                            println!("hit right edge for region {} at {},{}", r, x1, y1);
                            side_count += 1;
                            direction = (direction + 1) % 4;
                            x1 -= 1;
                        }
                        x1 += 1;
                    }

                    if direction == 1 {
                        // going down
                        if y1 >= regions.len() - 1 || regions[y1][x1] != *r {
                            println!("hit bottom edge for region {} at {},{}", r, x1, y1);
                            side_count += 1;
                            direction = (direction + 1) % 4;
                            y1 -= 1;
                            x1 -= 1;
                        }
                        y1 += 1;
                    }
                    if direction == 2 {
                        // going left
                        if x1 == 0 || regions[y1][x1] != *r {
                            println!("hit left edge for region {} at {},{}", r, x1, y1);
                            side_count += 1;
                            direction = (direction + 1) % 4;
                            x1 += 1;
                            y1 -= 1;
                        }
                        x1 -= 1;
                    }

                    if direction == 3 {
                        // going up
                        if y1 == 0 || regions[y1][x1] != *r {
                            println!("hit top edge for region {} at {},{}", r, x1, y1);
                            side_count += 1;
                            direction = (direction + 1) % 4;
                            y1 += 1;
                            x1 += 1;
                        }
                        y1 -= 1;
                    }
                }

                // always start at the top-most, left-most point

                side_counts.insert(*r, side_count);
            }
        }

        for i in 1..areas.len() + 1 {
            let r = &(i as i64);
            println!(
                "region: {} ({}), area: {}, sides: {}, price: {}",
                region_names[r],
                r,
                areas[r],
                side_counts[r],
                areas[r] * side_counts[r]
            );
        }

        let result: i64 = areas.iter().map(|(r, area)| area * side_counts[r]).sum();

        result
    }
}

fn convert_to_regions(chars: &Vec<Vec<char>>) -> Vec<Vec<i64>> {
    let mut regions: Vec<Vec<i64>> = chars
        .iter()
        .map(|row| row.iter().map(|c| -1i64).collect())
        .collect();
    let mut last_region_id: i64 = 0;

    let mut next_region_id = || {
        last_region_id += 1;
        last_region_id
    };

    for (y, row) in chars.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if regions[y][x] == -1 {
                let region_id = next_region_id();
                floodfill(&mut regions, x, y, region_id, &chars);
            }
        }
    }

    regions
}

fn floodfill(
    regions: &mut Vec<Vec<i64>>,
    x: usize,
    y: usize,
    region_id: i64,
    chars: &Vec<Vec<char>>,
) {
    regions[y][x] = region_id;

    if x > 0 && regions[y][x - 1] == -1 && chars[y][x - 1] == chars[y][x] {
        floodfill(regions, x - 1, y, region_id, chars);
    }

    if y > 0 && regions[y - 1][x] == -1 && chars[y - 1][x] == chars[y][x] {
        floodfill(regions, x, y - 1, region_id, chars);
    }

    if x < regions[y].len() - 1 && regions[y][x + 1] == -1 && chars[y][x + 1] == chars[y][x] {
        floodfill(regions, x + 1, y, region_id, chars);
    }

    if y < regions.len() - 1 && regions[y + 1][x] == -1 && chars[y + 1][x] == chars[y][x] {
        floodfill(regions, x, y + 1, region_id, chars);
    }
}
