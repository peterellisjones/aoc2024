use nom::{
    Finish,
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, line_ending, space1},
    error::Error as NomError,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};

use rayon::prelude::*;

use crate::Day;

pub struct Day7;

impl Day for Day7 {
    const DAY_NUMBER: i64 = 7;
    const PART1_EXAMPLE_SOLUTION: i64 = 3749;
    const PART2_EXAMPLE_SOLUTION: i64 = 11387;

    fn part1(raw_input: &str) -> i64 {
        parse(raw_input)
            .unwrap()
            .par_iter()
            .filter(|(target, inputs)| has_solution(*target, 0i64, inputs, false))
            .map(|(target, _)| target)
            .sum::<i64>()
    }

    fn part2(raw_input: &str) -> i64 {
        parse(raw_input)
            .unwrap()
            .par_iter()
            .filter(|(target, inputs)| has_solution(*target, 0i64, inputs, true))
            .map(|(target, _)| target)
            .sum::<i64>()
    }
}

fn has_solution(
    target: i64,
    running_total: i64,
    inputs: &[i64],
    allow_concatenation: bool,
) -> bool {
    // if there are no inputs left then we have a solution if the running total equals the target
    if inputs.len() == 0 {
        return target == running_total;
    }

    // if running_total is greater than the target then we have overshot
    if running_total > target {
        return false;
    }

    // otherwise we try a concatenation, multiplication and an addition to see if either provides a solution
    if allow_concatenation {
        let operand = inputs[0];
        let num_digits = 1 + (operand as f64).log10() as u32;
        let new_running_total = running_total * 10i64.pow(num_digits) + operand;

        if has_solution(target, new_running_total, &inputs[1..], allow_concatenation) {
            return true;
        }
    }

    // multiplication
    if has_solution(
        target,
        running_total * inputs[0],
        &inputs[1..],
        allow_concatenation,
    ) {
        return true;
    }

    // addition
    if has_solution(
        target,
        running_total + inputs[0],
        &inputs[1..],
        allow_concatenation,
    ) {
        return true;
    }

    false
}

fn parse(raw_input: &str) -> Result<Vec<(i64, Vec<i64>)>, NomError<&str>> {
    many1(terminated(
        separated_pair(
            terminated(nom_i64, tag(":")),
            space1,
            separated_list1(space1, nom_i64),
        ),
        line_ending,
    ))(raw_input)
    .finish()
    .map(|(_, x)| x)
}
