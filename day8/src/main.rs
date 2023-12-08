use std::{collections::HashMap, fs::read_to_string, iter::Cycle, str::Chars};

use nom::{
    bytes::complete::{tag, take_while},
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

#[derive(Debug)]
struct Directions<'a>(&'a str);

struct DirectionsIntoIter<'a>(Cycle<Chars<'a>>);

impl<'a> IntoIterator for Directions<'a> {
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
    let mut input = input.split("\n\n").collect::<Vec<_>>();
    let directions = Directions::from(input[0]);
    assert_eq!(input.len(), 2);
    let mut graph: HashMap<Tag, Node> = HashMap::new();
    while let Ok((input1, node)) = parse_node(input[1]) {
        graph.insert(node.0, node.1);
        input[1] = input1;
    }
    let mut position = Tag("AAA");
    let mut steps = 0;
    for direction in directions {
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
}
