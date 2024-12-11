use std::fs::read_to_string;

use nom::{
    Finish,
    character::complete::{i64 as nom_i64, line_ending, multispace1, not_line_ending, space1},
    error::Error as NomError,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};

pub fn read_input(problem_number: i64, example: Option<i64>) -> Result<String, std::io::Error> {
    let file_path = if let Some(part) = example {
        format!("assets/day{problem_number}_part{part}_example.txt")
    } else {
        format!("assets/day{problem_number}.txt")
    };

    read_to_string(file_path)
}

pub fn parse_integer_pairs(input: &str) -> Result<Vec<(i64, i64)>, NomError<&str>> {
    many1(terminated(
        separated_pair(nom_i64, multispace1, nom_i64),
        line_ending,
    ))(input)
    .finish()
    .map(|(_, x)| x)
}

pub fn parse_integer_list(input: &str) -> Result<Vec<Vec<i64>>, NomError<&str>> {
    many1(terminated(separated_list1(space1, nom_i64), line_ending))(input)
        .finish()
        .map(|(_, x)| x)
}

pub fn parsed_space_separated_integers(input: &str) -> Result<Vec<i64>, NomError<&str>> {
    terminated(separated_list1(space1, nom_i64), line_ending)(input)
        .finish()
        .map(|(_, x)| x)
}

pub fn parse_char_grid(input: &str) -> Result<Vec<Vec<char>>, NomError<&str>> {
    parse_grid(input, |c| c)
}

pub fn parse_digit_grid(input: &str) -> Result<Vec<Vec<i32>>, NomError<&str>> {
    parse_grid(input, |c| c.to_digit(10).unwrap() as i32)
}

pub fn parse_grid<T, F: Fn(char) -> T>(
    input: &str,
    parse_char: F,
) -> Result<Vec<Vec<T>>, NomError<&str>> {
    many1(terminated(not_line_ending, line_ending))(input)
        .finish()
        .map(|(_, x)| {
            x.iter()
                .map(|&y| y.chars().map(|c| parse_char(c)).collect::<Vec<T>>())
                .collect::<Vec<Vec<T>>>()
        })
}

pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn width(self: &Self) -> usize {
        self.0[0].len()
    }

    pub fn height(self: &Self) -> usize {
        self.0.len()
    }

    pub fn for_each_neighbour<F: FnMut(usize, usize, &T)>(
        self: &Self,
        y: usize,
        x: usize,
        mut callback: F,
    ) {
        if y > 0 {
            callback(y - 1, x, &self.0[y - 1][x]);
        }
        if y < self.height() - 1 {
            callback(y + 1, x, &self.0[y + 1][x]);
        }
        if x > 0 {
            callback(y, x - 1, &self.0[y][x - 1]);
        }
        if x < self.width() - 1 {
            callback(y, x + 1, &self.0[y][x + 1]);
        }
    }
}
