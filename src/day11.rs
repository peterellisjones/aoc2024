use fnv::FnvHashMap;

use crate::{Day, utils::*};

pub struct Day11;

impl Day for Day11 {
    const DAY_NUMBER: i64 = 11;
    const PART1_EXAMPLE_SOLUTION: i64 = 55312;
    const PART2_EXAMPLE_SOLUTION: i64 = 65601038650482;

    fn part1(raw_input: &str) -> i64 {
        solve(raw_input, 25)
    }

    fn part2(raw_input: &str) -> i64 {
        solve(raw_input, 75)
    }
}

fn solve(raw_input: &str, num_blinks: usize) -> i64 {
    let stones = parsed_space_separated_integers(raw_input).unwrap();

    // maps stone, number of blinks to total stones
    let mut cache: FnvHashMap<(i64, usize), i64> = FnvHashMap::default();
    cache.insert((0, 1), 1);

    stones
        .iter()
        .map(|stone| blink_on_stone_n_times(*stone, num_blinks, &mut cache))
        .sum()
}

fn blink_on_stone_n_times(
    stone: i64,
    times: usize,
    mut cache: &mut FnvHashMap<(i64, usize), i64>,
) -> i64 {
    if times == 0 {
        return 1;
    }

    if let Some(num_stones) = cache.get(&(stone, times)) {
        return *num_stones;
    }

    // 1. If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
    let num_stones = if stone == 0 {
        blink_on_stone_n_times(1, times - 1, &mut cache)
    } else {
        // 2. If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
        // The left half of the digits are engraved on the new left stone, and the right half of the digits are
        // engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
        let num_digits = 1 + stone.ilog10();
        if num_digits % 2 == 0 {
            let divisor = 10_i64.pow(num_digits / 2);
            blink_on_stone_n_times(stone / divisor, times - 1, &mut cache)
                + blink_on_stone_n_times(stone % divisor, times - 1, &mut cache)
        } else {
            // 3. If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
            blink_on_stone_n_times(stone * 2024, times - 1, &mut cache)
        }
    };

    cache.insert((stone, times), num_stones);

    num_stones
}
