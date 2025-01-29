use nom::{
    Finish,
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, line_ending},
    error::Error as NomError,
    multi::{many0, many1},
    sequence::{pair, preceded, separated_pair, terminated},
};

use crate::Day;

pub struct Day13;

impl Day for Day13 {
    const DAY_NUMBER: i64 = 13;
    const PART1_EXAMPLE_SOLUTION: i64 = 480;
    const PART2_EXAMPLE_SOLUTION: i64 = 875318608908;

    fn part1(raw_input: &str) -> i64 {
        let machines = parse(raw_input, 0).unwrap();

        let mut total_cost = 0i64;

        for machine in machines.iter() {
            if let Some(solution) = machine.solution() {
                total_cost += solution.0 * 3 + solution.1;
            }
        }

        total_cost
    }

    fn part2(raw_input: &str) -> i64 {
        let machines = parse(raw_input, 10000000000000).unwrap();

        let mut total_cost = 0i64;

        for machine in machines.iter() {
            if let Some(solution) = machine.solution() {
                total_cost += solution.0 * 3 + solution.1;
            }
        }

        total_cost
    }
}

#[derive(Debug)]
struct Machine {
    button_a_x: i64,
    button_a_y: i64,
    button_b_x: i64,
    button_b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

impl Machine {
    fn solution(&self) -> Option<(i64, i64)> {
        let numerator = self.button_a_y * self.prize_x - self.button_a_x * self.prize_y;
        let denominator = self.button_a_y * self.button_b_x - self.button_a_x * self.button_b_y;

        if numerator % denominator != 0 {
            return None;
        }
        let num_bs = numerator / denominator;

        if (self.prize_x - self.button_b_x * num_bs) % self.button_a_x != 0 {
            return None;
        }
        let num_as = (self.prize_x - self.button_b_x * num_bs) / self.button_a_x;

        Some((num_as, num_bs))
    }
}

fn parse(input: &str, extra_prize_cost: i64) -> Result<Vec<Machine>, NomError<&str>> {
    // let machines = Vec::new();

    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400

    let button_a = terminated(
        preceded(
            tag("Button A: X+"),
            separated_pair(nom_i64, tag(", Y+"), nom_i64),
        ),
        line_ending,
    );

    let button_b = terminated(
        preceded(
            tag("Button B: X+"),
            separated_pair(nom_i64, tag(", Y+"), nom_i64),
        ),
        line_ending,
    );

    let prize = terminated(
        preceded(
            tag("Prize: X="),
            separated_pair(nom_i64, tag(", Y="), nom_i64),
        ),
        line_ending,
    );

    many1(terminated(
        pair(pair(button_a, button_b), prize),
        many0(line_ending),
    ))(input)
    .finish()
    .map(|(_, x)| {
        x.iter()
            .map(|(((ax, ay), (bx, by)), (px, py))| Machine {
                button_a_x: *ax,
                button_a_y: *ay,
                button_b_x: *bx,
                button_b_y: *by,
                prize_x: *px + extra_prize_cost,
                prize_y: *py + extra_prize_cost,
            })
            .collect()
    })
}
