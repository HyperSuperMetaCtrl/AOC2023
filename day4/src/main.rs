use anyhow::Result;
use nom::{bytes::complete::*, character::complete::*, multi::*, sequence::*, IResult};
use std::fs::read_to_string;

#[derive(Debug)]
struct Round {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl From<(u32, Vec<u32>, Vec<u32>)> for Round {
    fn from(tuple: (u32, Vec<u32>, Vec<u32>)) -> Self {
        Self {
            winning_numbers: tuple.1,
            my_numbers: tuple.2,
        }
    }
}

impl Round {
    fn score(self) -> u32 {
        let num_winning: u32 = self
            .winning_numbers
            .into_iter()
            .fold(0u32, |acc, w| {
                if self.my_numbers.contains(&w) {
                    acc + 1
                } else {
                    acc
                }
            });
        if num_winning > 0 {
            1 << (num_winning - 1)
        } else {
            0
        }
    }
}

fn parse_card(input: &str) -> IResult<&str, u32> {
    let (input, card_id) = terminated(
        preceded(tuple((tag("Card"), many1(tag(" ")))), u32),
        tag(":"),
    )(input)?;
    Ok((input, card_id))
}

fn parse_winning_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, winning_numbers) = terminated(
        preceded(many0(tag(" ")), separated_list1(many1(tag(" ")), u32)),
        tag(" | "),
    )(input)?;
    Ok((input, winning_numbers))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, numbers) = terminated(
        preceded(many0(tag(" ")), separated_list1(many1(tag(" ")), u32)),
        tag("\n"),
    )(input)?;
    Ok((input, numbers))
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, parsed) = tuple((parse_card, parse_winning_numbers, parse_numbers))(input)?;
    Ok((input, Round::from(parsed)))
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let mut input = input.as_str();
    let mut sum = 0;
    while let Ok((input1, round)) = parse_round(input) {
        sum += round.score();
        input = input1;
    }
    println!("Day 4 Part 1: {sum}");

    Ok(())
}
