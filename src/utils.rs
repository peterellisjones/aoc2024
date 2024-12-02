use std::fs::read_to_string;

use nom::{
    Finish,
    character::complete::{i64 as nom_i64, line_ending, multispace1, space1},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};

pub fn read_input(problem_number: i64, example: bool) -> Result<String, std::io::Error> {
    let file_path = if example {
        format!("assets/day{problem_number}_example.txt")
    } else {
        format!("assets/day{problem_number}.txt")
    };

    read_to_string(file_path)
}

pub fn parse_integer_pairs(input: &str) -> Result<Vec<(i64, i64)>, nom::error::Error<&str>> {
    many1(terminated(
        separated_pair(nom_i64, multispace1, nom_i64),
        line_ending,
    ))(input)
    .finish()
    .map(|(_, x)| x)
}

pub fn parse_integer_list(input: &str) -> Result<Vec<Vec<i64>>, nom::error::Error<&str>> {
    many1(terminated(separated_list1(space1, nom_i64), line_ending))(input)
        .finish()
        .map(|(_, x)| x)
}
