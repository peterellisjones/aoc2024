use core::num;

use crate::{Day, utils::parse_char_grid};

pub struct Day4;

impl Day for Day4 {
    const DAY_NUMBER: i64 = 4;
    const PART1_EXAMPLE_SOLUTION: i64 = 18;
    const PART2_EXAMPLE_SOLUTION: i64 = 9;

    fn part1(raw_input: &str) -> i64 {
        let grid = parse_char_grid(raw_input).unwrap();

        let mut num_xmas = 0;

        const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

        let height = grid.len();
        let width = grid[0].len();

        // horizontal
        for y in 0..height {
            for x in 0..(width - 3) {
                // forward
                let mut found = true;
                for i in 0..4 {
                    if grid[y][x + i] != XMAS[i] {
                        found = false;
                        break;
                    }
                }

                if found {
                    num_xmas += 1;
                }

                // backward
                let mut found = true;
                for i in 0..4 {
                    if grid[y][x + i] != XMAS[3 - i] {
                        found = false;
                        break;
                    }
                }

                if found {
                    num_xmas += 1;
                }
            }
        }

        // vertical
        for y in 0..(height - 3) {
            for x in 0..width {
                // forward
                let mut found = true;
                for i in 0..4 {
                    if grid[y + i][x] != XMAS[i] {
                        found = false;
                        break;
                    }
                }

                if found {
                    num_xmas += 1;
                }

                // backward
                let mut found = true;
                for i in 0..4 {
                    if grid[y + i][x] != XMAS[3 - i] {
                        found = false;
                        break;
                    }
                }

                if found {
                    num_xmas += 1;
                }
            }
        }

        // diagonal top-left to bottom-right
        for y in 0..(height - 3) {
            for x in 0..(width - 3) {
                // forward
                let mut found = true;
                for i in 0..4 {
                    if grid[y + i][x + i] != XMAS[i] {
                        found = false;
                        break;
                    }
                }

                if found {
                    num_xmas += 1;
                }

                // backward
                let mut found = true;
                for i in 0..4 {
                    if grid[y + i][x + i] != XMAS[3 - i] {
                        found = false;
                        break;
                    }
                }

                if found {
                    num_xmas += 1;
                }
            }
        }

        // diagonal top-right to bottom-left
        for y in 0..(height - 3) {
            for x in 0..(width - 3) {
                // forward
                let mut found = true;
                for i in 0..4 {
                    if grid[y + i][x + 3 - i] != XMAS[i] {
                        found = false;
                        break;
                    }
                }

                if found {
                    num_xmas += 1;
                }

                // backward
                let mut found = true;
                for i in 0..4 {
                    if grid[y + i][x + 3 - i] != XMAS[3 - i] {
                        found = false;
                        break;
                    }
                }

                if found {
                    num_xmas += 1;
                }
            }
        }

        num_xmas
    }

    fn part2(raw_input: &str) -> i64 {
        let grid = parse_char_grid(raw_input).unwrap();

        let mut num_x_mas = 0;

        const X_MAS: [[char; 3]; 3] = [['M', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'S']];

        let height = grid.len();
        let width = grid[0].len();

        for y in 0..(height - 2) {
            for x in 0..(width - 2) {
                // forward
                let mut found = true;
                for i in 0..3 {
                    for j in 0..3 {
                        if X_MAS[j][i] != '.' {
                            if grid[y + j][x + i] != X_MAS[j][i] {
                                found = false;
                                break;
                            }
                        }
                    }

                    if !found {
                        break;
                    }
                }

                if found {
                    num_x_mas += 1;
                }

                // backward
                let mut found = true;
                for i in 0..3 {
                    for j in 0..3 {
                        if X_MAS[j][2 - i] != '.' {
                            if grid[y + j][x + i] != X_MAS[j][2 - i] {
                                found = false;
                                break;
                            }
                        }
                    }

                    if !found {
                        break;
                    }
                }

                if found {
                    num_x_mas += 1;
                }

                // up
                let mut found = true;
                for i in 0..3 {
                    for j in 0..3 {
                        if X_MAS[i][j] != '.' {
                            if grid[y + j][x + i] != X_MAS[i][j] {
                                found = false;
                                break;
                            }
                        }
                    }

                    if !found {
                        break;
                    }
                }

                if found {
                    num_x_mas += 1;
                }

                // down
                let mut found = true;
                for i in 0..3 {
                    for j in 0..3 {
                        if X_MAS[i][2 - j] != '.' {
                            if grid[y + j][x + i] != X_MAS[i][2 - j] {
                                found = false;
                                break;
                            }
                        }
                    }

                    if !found {
                        break;
                    }
                }

                if found {
                    num_x_mas += 1;
                }
            }
        }

        num_x_mas
    }
}
