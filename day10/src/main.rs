use anyhow::{anyhow, Result};
use ndarray::Array;
use std::fs::read_to_string;
use day10::{Tile, Walker, Point};

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let width = input.find("\n").ok_or(anyhow!("no newline found"))?;
    let input = input.replace("\n", "");
    let input: Vec<Tile> = input.chars().filter_map(|x| Tile::try_from(x).ok()).collect();
    let pipe_maze = Array::from_shape_vec((width, width), input.into())?;
    let (start, _) = pipe_maze.indexed_iter().find(|x| *x.1 == Tile::S).ok_or(anyhow!("start not found"))?;
    // find starting orientations
    let start = Point { x: start.1, y: start.0 };
    let mut walkers = Walker::new(&pipe_maze, start);
    let mut counter = 0;
    loop {
        walkers.0.walk();
        walkers.1.walk();
        counter += 1;

        if walkers.0.position() == walkers.1.position() {
            break;
        }
    }
    println!("Day 10 Part 2: {counter}");
    Ok(())
}
