use nom::{
    Finish,
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, line_ending, space1},
    error::Error as NomError,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};

use crate::Day;

pub struct Day7;

impl Day for Day7 {
    const DAY_NUMBER: i64 = 7;
    const PART1_EXAMPLE_SOLUTION: i64 = 3749;
    const PART2_EXAMPLE_SOLUTION: i64 = -1;

    fn part1(raw_input: &str) -> i64 {
        let equations = parse(raw_input).unwrap();

        -1
    }

    fn part2(raw_input: &str) -> i64 {
        unimplemented!();
    }
}

type Equation = (i64, Vec<i64>);

fn parse(raw_input: &str) -> Result<Vec<Equation>, NomError<&str>> {
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
