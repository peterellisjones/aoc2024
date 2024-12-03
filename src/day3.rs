use crate::Day;

use regex::Regex;

pub struct Day3;

impl Day for Day3 {
    const DAY_NUMBER: i64 = 3;
    const PART1_EXAMPLE_SOLUTION: i64 = 161;
    const PART2_EXAMPLE_SOLUTION: i64 = 48;

    fn part1(raw_input: &str) -> i64 {
        solve(raw_input, true)
    }

    fn part2(raw_input: &str) -> i64 {
        solve(raw_input, false)
    }
}

fn solve(raw_input: &str, ignore_dos_and_donts: bool) -> i64 {
    let match_operation = Regex::new(r"(mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\))").unwrap();
    let match_mul = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    #[derive(Debug)]
    enum Op {
        Do,
        Dont,
        Mul(i64, i64),
    }

    let (sum, _) = match_operation
        .find_iter(raw_input)
        .map(|reg_match| {
            let operation = reg_match.as_str();

            if let Some(captures) = match_mul.captures(operation) {
                let (_, operands): (&str, [&str; 2]) = captures.extract();

                Op::Mul(
                    operands[0].parse::<i64>().unwrap(),
                    operands[1].parse::<i64>().unwrap(),
                )
            } else if operation == "do()" {
                Op::Do
            } else if operation == "don't()" {
                Op::Dont
            } else {
                unreachable!()
            }
        })
        .fold(
            (0i64, true),
            |(sum, mul_enabled), operation| match operation {
                Op::Do => (sum, true),
                Op::Dont => (sum, false || ignore_dos_and_donts),
                Op::Mul(left, right) => {
                    if mul_enabled {
                        (sum + left * right, mul_enabled)
                    } else {
                        (sum, mul_enabled)
                    }
                }
            },
        );

    sum
}
