use anyhow::Result;
use std::collections::HashMap;
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

    // define a lookup table
    let lookup_nums = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut sum = 0;
    for line in input.lines() {
        let left_index = line.find(|c: char| c.is_ascii_digit()).unwrap();
        let right_index = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
        let mut num = 0;
        if let Some((_, left)) = find_replacement(&lookup_nums, &line[..left_index]) {
            num += 10 * left;
        } else {
            num += 10 * line.chars().nth(left_index).unwrap().to_digit(10).unwrap();
        }
        if let Some((_, right)) = rfind_replacement(&lookup_nums, &line[right_index..]) {
            num += right;
        } else {
            num += line.chars().nth(right_index).unwrap().to_digit(10).unwrap();
        }
        sum += num;
    }
    println!("Day 1 Part 2: {sum}");
    Ok(())
}

fn find_replacement<'a>(
    lookup: &'a HashMap<&'a str, u32>,
    substr: &'a str,
) -> Option<(usize, &'a u32)> {
    let keys: Vec<_> = lookup.keys().collect();
    let mut replacements = Vec::new();
    for key in keys {
        if let Some(index) = substr.find(key) {
            replacements.push((index, lookup.get(key).unwrap()))
        }
    }
    replacements.into_iter().min_by(|x, y| x.0.cmp(&y.0))
}

fn rfind_replacement<'a>(
    lookup: &'a HashMap<&'a str, u32>,
    substr: &'a str,
) -> Option<(usize, &'a u32)> {
    let keys: Vec<_> = lookup.keys().collect();
    let mut replacements = Vec::new();
    for key in keys {
        if let Some(index) = substr.rfind(key) {
            replacements.push((index, lookup.get(key).unwrap()))
        }
    }
    replacements.into_iter().max_by(|x, y| x.0.cmp(&y.0))
}
