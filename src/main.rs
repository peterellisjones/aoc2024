use day1::Day1;

mod utils;

mod day1;

pub trait Day {
    const DAY_NUMBER: i64;
    const PART1_EXAMPLE_SOLUTION: i64;
    const PART2_EXAMPLE_SOLUTION: i64;

    fn part1(raw_input: &str) -> i64;
    fn part2(raw_input: &str) -> i64;

    fn run() {
        let example_input = &utils::read_input(Self::DAY_NUMBER, true).unwrap();
        let intput = &utils::read_input(Self::DAY_NUMBER, false).unwrap();

        debug_assert_eq!(Self::part1(example_input), Self::PART1_EXAMPLE_SOLUTION);

        let part1_solution = Self::part1(intput);
        println!("day {} part 1: {}", Self::DAY_NUMBER, part1_solution);

        debug_assert_eq!(Self::part2(example_input), Self::PART2_EXAMPLE_SOLUTION);

        let part2_solution = Self::part2(intput);
        println!("day {} part 2: {}", Self::DAY_NUMBER, part2_solution);
    }
}

fn main() {
    Day1::run();
}
