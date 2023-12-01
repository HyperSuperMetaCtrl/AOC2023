use anyhow::Result;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;

    // for every line extract numbers
    let numbers: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10))
                .collect::<Vec<char>>()
        })
        .collect();
    // for every line find the first and last digit
    let numbers: Vec<_> = numbers
        .into_iter()
        .map(|number| {
            number.first().unwrap().to_digit(10).unwrap() * 10
                + number.last().unwrap().to_digit(10).unwrap()
        })
        .collect();
    // add them all up
    let sum: u32 = numbers.iter().sum();

    println!("Day 1 Part 1: {sum}");
    Ok(())
}
