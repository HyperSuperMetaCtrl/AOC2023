use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use std::{collections::HashMap, fs::read_to_string, iter::Cycle, str::Chars};

use nom::{
    bytes::complete::{tag, take_while},
    multi::many0,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tag<'a>(&'a str);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node<'a>(Tag<'a>, Tag<'a>);

impl<'a> Node<'a> {
    fn left(&'a self) -> Tag<'a> {
        self.0
    }
    fn right(&'a self) -> Tag<'a> {
        self.1
    }
}
#[derive(Debug, Copy, Clone)]
enum Direction {
    L,
    R,
}

#[derive(Debug, Clone)]
struct Directions<'a>(&'a str);

struct DirectionsIntoIter<'a>(Cycle<Chars<'a>>);

impl<'a> Directions<'a> {
    fn iter(&self) -> DirectionsIntoIter<'_> {
        self.into_iter()
    }
}
impl<'a> IntoIterator for Directions<'a> {
    type Item = Direction;

    type IntoIter = DirectionsIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DirectionsIntoIter(self.0.chars().cycle())
    }
}

impl<'a> IntoIterator for &Directions<'a> {
    type Item = Direction;

    type IntoIter = DirectionsIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DirectionsIntoIter(self.0.chars().cycle())
    }
}

impl<'a> Iterator for DirectionsIntoIter<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some('L') => Some(Self::Item::L),
            Some('R') => Some(Self::Item::R),
            _ => None,
        }
    }
}

impl<'a> From<&'a str> for Directions<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

fn parse_tag(input: &str) -> IResult<&str, Tag> {
    let (input, tag) = take_while(char::is_alphabetic)(input)?;
    Ok((input, Tag(tag)))
}

fn parse_node(input: &str) -> IResult<&str, (Tag, Node)> {
    let (input, node_tag) = parse_tag(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = parse_tag(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = parse_tag(input)?;
    let (input, _) = tag(")\n")(input)?;
    Ok((input, (node_tag, Node(left, right))))
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.split("\n\n").collect::<Vec<_>>();
    let directions = Directions::from(input[0]);
    let directions_part2 = directions.clone();
    assert_eq!(input.len(), 2);
    let mut graph: HashMap<Tag, Node> = HashMap::new();
    let (input, nodes) = many0(parse_node)(input[1]).unwrap();
    assert_eq!(input, "");
    nodes.iter().for_each(|x| {
        graph.insert(x.0, x.1);
    });
    let mut position = Tag("AAA");
    let mut steps = 0;
    for direction in &directions {
        if position == Tag("ZZZ") {
            break;
        }
        let node = graph.get(&position).unwrap();
        position = match direction {
            Direction::L => node.left(),
            Direction::R => node.right(),
        };
        steps += 1;
    }
    println!("Day 8 Part 1: {steps}");
    // let mut positions: Vec<Tag> = graph
    //     .keys()
    //     .into_iter()
    //     .cloned()
    //     .filter(|x| x.0.chars().last().unwrap() == 'A')
    //     .collect();
    // dbg!(positions.len());
    // let mut flags = vec![false; positions.len()];
    // steps = 0;
    // for direction in &directions_part2 {
    //     for (index, position) in positions.iter_mut().enumerate() {
    //         if position.0.chars().last().unwrap() == 'Z' {
    //             flags[index] = true;
    //         }
    //         let node = graph.get(&position).unwrap();
    //         *position = match direction {
    //             Direction::L => node.left(),
    //             Direction::R => node.right(),
    //         };
    //     }
    //     if flags.iter().all(|x| *x == true) {
    //         break;
    //     }
    //     steps += 1;
    //     flags.iter_mut().for_each(|x| *x = false);
    // }

    // println!("Day 8 Part 2: {steps}");
}
