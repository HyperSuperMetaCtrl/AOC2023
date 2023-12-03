use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use std::fs::read_to_string;

const NUM_RED: u32 = 12;
const NUM_GREEN: u32 = 13;
const NUM_BLUE: u32 = 14;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}
impl TryFrom<&str> for Color {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err("Could not convert &str to Color"),
        }
    }
}
#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn add(&mut self, amount: u32, color: Color) {
        match color {
            Color::Red => self.red += amount,
            Color::Green => self.green += amount,
            Color::Blue => self.blue += amount,
        }
    }
    fn is_valid(&self) -> bool {
        if self.red <= NUM_RED && self.green <= NUM_GREEN && self.blue <= NUM_BLUE {
            true
        } else {
            false
        }
    }
    fn assign_if_larger(&mut self, round: &Round) {
        if self.red < round.red {
            self.red = round.red
        }
        if self.green < round.green {
            self.green = round.green
        }
        if self.blue < round.blue {
            self.blue = round.blue
        }
    }
    fn cube(&self) -> u32 {
       self.red * self.green * self.blue
    }
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, color) = alt((tag("red"), tag("green"), tag("blue")))(input)?;
    Ok((input, Color::try_from(color).unwrap()))
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let mut round = Round::new();
    let (input, pairs) = terminated(
        separated_list1(tag(", "), separated_pair(u32, tag(" "), parse_color)),
        alt((tag("; "), tag("\n"))),
    )(input)?;
    for pair in pairs {
        round.add(pair.0, pair.1);
    }
    Ok((input, round))
}

fn parse_game_id(input: &str) -> IResult<&str, u32> {
    let (input, (_, game_id, _)) = tuple((tag("Game "), u32, tag(": ")))(input)?;
    Ok((input, game_id))
}

fn part1(mut input: &str) {
    let mut sum = 0;
    loop {
        let Ok((input1, game_id)) = parse_game_id(&input) else {
            break;
        };
        input = input1;
        let mut game_valid = true;
        while let Ok((input1, round)) = parse_round(input) {
            if !round.is_valid() {
                game_valid = false;
            }
            input = input1
        }
        if game_valid {
            sum += game_id;
        }
    }
    println!("Day 2 Part 1: {sum}");
}

fn part2(mut input: &str) {
    let mut sum = 0;
    loop {
        let mut min_cubes = Round::new();
        let Ok((input1, _)) = parse_game_id(&input) else {
            break;
        };
        input = input1;
        while let Ok((input1, round)) = parse_round(input) {
            min_cubes.assign_if_larger(&round);
            input = input1
        }
        sum += min_cubes.cube();
    }
    println!("Day 2 Part 2: {sum}");
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    part1(&input);
    part2(&input);

    Ok(())
}
