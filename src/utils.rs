use std::fs::read_to_string;

use nom::{
    Finish, IResult,
    character::complete::{i64 as nom_i64, line_ending, multispace1},
    multi::many1,
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
    let parse_pair = separated_pair(nom_i64, multispace1, nom_i64);

    let parser: IResult<&str, Vec<(i64, i64)>> = many1(terminated(parse_pair, line_ending))(input);

    parser.finish().map(|(_, x)| x)
}
