use anyhow::Result;
use nom::{bytes::complete::*, character::complete::*, multi::*, sequence::*, IResult};
use std::fs::read_to_string;

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl From<(Vec<u32>, Vec<u32>)> for Card {
    fn from(tuple: (Vec<u32>, Vec<u32>)) -> Self {
        Self {
            winning_numbers: tuple.0,
            my_numbers: tuple.1,
        }
    }
}

impl Card {
    fn score(&self, double: bool) -> u32 {
        let num_winning: u32 = self.winning_numbers.iter().fold(0u32, |acc, w| {
            if self.my_numbers.contains(&w) {
                acc + 1
            } else {
                acc
            }
        });
        if num_winning > 0 && double {
            1 << (num_winning - 1)
        } else {
            num_winning
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

fn parse_round(input: &str) -> IResult<&str, Card> {
    let (input, parsed) = preceded(parse_card, tuple((parse_winning_numbers, parse_numbers)))(input)?;
    Ok((input, Card::from(parsed)))
}

fn part1(mut input: &str) {
    let mut sum = 0;
    while let Ok((input1, round)) = parse_round(input) {
        sum += round.score(true);
        input = input1;
    }
    println!("Day 4 Part 1: {sum}");
}

fn part2(mut input: &str) {
    let mut all_cards: Vec<Card> = vec![];
    while let Ok((input1, card)) = parse_round(input) {
        all_cards.push(card);
        input = input1;
    }
    let mut card_counts = vec![1; all_cards.len()];
    for (index, card) in all_cards.iter().enumerate() {
        let score = card.score(false);
            for i in index + 1..=index + score as usize {
                card_counts[i] += card_counts[index];
            }
    }
    println!("Day 4 Part 2: {}", card_counts.iter().sum::<u32>());
}

fn main() -> Result<()> {
    let mut input = read_to_string("input.txt")?;
    part1(input.clone().as_mut_str());
    part2(input.as_mut_str());
    Ok(())
}
