use std::time::Instant;

mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    day1::Day1::run();
    day2::Day2::run();
    day3::Day3::run();
    day4::Day4::run();
    day5::Day5::run();
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
        let example_part1_input = &utils::read_input(Self::DAY_NUMBER, Some(1)).unwrap();
        let example_part2_input = &utils::read_input(Self::DAY_NUMBER, Some(2)).unwrap();
        let input = &utils::read_input(Self::DAY_NUMBER, None).unwrap();

        let now = Instant::now();

        assert_eq!(
            Self::part1(example_part1_input),
            Self::PART1_EXAMPLE_SOLUTION
        );

        println!("day {}:", Self::DAY_NUMBER);

        let part1_solution = Self::part1(input);
        println!("\tpart 1: {}", part1_solution);

        assert_eq!(
            Self::part2(example_part2_input),
            Self::PART2_EXAMPLE_SOLUTION
        );

        let part2_solution = Self::part2(input);
        println!("\tpart 2: {}", part2_solution);

        let elapsed = now.elapsed();

        println!("\ttime: {:.2?}", elapsed);
    }
}
