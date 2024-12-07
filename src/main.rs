use std::{env::args, time::Instant};

mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    #[derive(PartialEq)]
    enum DayToRun {
        Day(i64),
        All,
    }

    let days = vec![
        day1::Day1::run,
        day2::Day2::run,
        day3::Day3::run,
        day4::Day4::run,
        day5::Day5::run,
        day6::Day6::run,
        day7::Day7::run,
    ];

    let mut day_to_run = DayToRun::All;
    for argument in args() {
        if let Some(d) = argument.parse().ok() {
            if (0..days.len()).contains(&((d - 1) as usize)) {
                day_to_run = DayToRun::Day(d);
            }
        }
    }

    if let DayToRun::Day(d) = day_to_run {
        days[(d - 1) as usize]();
    } else {
        for day in days {
            day();
        }
    }
}

pub trait Day {
    const DAY_NUMBER: i64;

    const PART1_EXAMPLE_SOLUTION: i64;
    const PART2_EXAMPLE_SOLUTION: i64;

    fn part1(_: &str) -> i64 {
        unimplemented!("day {} part 1 not yet implemented", Self::DAY_NUMBER);
    }

    fn part2(_: &str) -> i64 {
        unimplemented!("day {} part 2 not yet implemented", Self::DAY_NUMBER);
    }

    fn run() {
        #[cfg(debug_assertions)]
        let example_part1_input = &utils::read_input(Self::DAY_NUMBER, Some(1)).unwrap();

        #[cfg(debug_assertions)]
        debug_assert_eq!(
            Self::part1(example_part1_input),
            Self::PART1_EXAMPLE_SOLUTION
        );

        let input: &String = &utils::read_input(Self::DAY_NUMBER, None).unwrap();

        let now = Instant::now();
        let part1_solution = Self::part1(input);
        let elapsed_part_1 = now.elapsed();

        #[cfg(debug_assertions)]
        let example_part2_input = &utils::read_input(Self::DAY_NUMBER, Some(2)).unwrap();

        #[cfg(debug_assertions)]
        assert_eq!(
            Self::part2(example_part2_input),
            Self::PART2_EXAMPLE_SOLUTION
        );

        let now = Instant::now();
        let part2_solution = Self::part2(input);
        let elapsed_part_2 = now.elapsed();

        println!("day {}:", Self::DAY_NUMBER);
        println!("\tpart 1: {}", part1_solution);
        println!("\tpart 2: {}", part2_solution);
        println!("\ttime: {:.2?}", elapsed_part_1 + elapsed_part_2);
    }
}
