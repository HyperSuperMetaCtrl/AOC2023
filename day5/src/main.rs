use rayon::prelude::*;
use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, one_of, space1, u64},
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};
use std::{fs::read_to_string, ops::Range, sync::{Arc, Mutex}};

static STRINGS: [&str; 7] = [
    "seed-to-soil map:\n",
    "soil-to-fertilizer map:\n",
    "fertilizer-to-water map:\n",
    "water-to-light map:\n",
    "light-to-temperature map:\n",
    "temperature-to-humidity map:\n",
    "humidity-to-location map:\n",
];

type PResult<'a, T> = IResult<&'a str, T>;

fn parse_seeds(input: &str) -> PResult<Vec<u64>> {
    preceded(tag("seeds: "), separated_list1(space1, u64))(input)
}

fn parse_map_nums(input: &str) -> PResult<Vec<u64>> {
    separated_list1(one_of(" \n"), u64)(input)
}

fn parse_map<'a>(input: &'a str, map_str: &'a str) -> PResult<'a, Vec<u64>> {
    preceded(many1(newline), preceded(tag(map_str), parse_map_nums))(input)
}

fn parse_almanac<'a>(input: &'a str) -> PResult<'a, Almanac> {
    let (mut input, seeds) = parse_seeds(input)?;
    let mut maps: Vec<Vec<u64>> = Vec::with_capacity(STRINGS.len());
    for map_str in STRINGS {
        let (input1, map) = parse_map(input, map_str)?;
        maps.push(map);
        input = input1;
    }

    Ok((input, Almanac::from((seeds, maps))))
}

#[derive(Default, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Vec<Vec<u64>>>,
}

impl Almanac {
    fn transform(&self, mut input: u64) -> u64 {
        for map in &self.maps {
            if let Some(pos) = map
                .par_iter()
                .position_any(|x| (x[1]..(x[1] + x[2])).contains(&input))
            {
                let offset = input - map[pos][1];
                input = map[pos][0] + offset;
            }
        }
        input
    }
    fn transform_slice(&self, input: &mut [u64]) {
        input.iter_mut().for_each(|e| *e = self.transform(*e));
    }
}

impl From<(Vec<u64>, Vec<Vec<u64>>)> for Almanac {
    fn from((seeds, maps): (Vec<u64>, Vec<Vec<u64>>)) -> Self {
        let maps: Vec<Vec<Vec<u64>>> = maps
            .into_iter()
            .map(|map| {
                map.into_iter()
                    .chunks(3)
                    .into_iter()
                    .map(|chunk| chunk.collect())
                    .collect()
            })
            .collect();
        Almanac { seeds, maps }
    }
}

fn main() -> Result<()> {
    let input: String = read_to_string("input.txt")?;
    let (input, almanac) = parse_almanac(&input).unwrap();
    assert!(input == "\n");
    let transformed: Vec<u64> = almanac
        .seeds
        .iter()
        .map(|x| almanac.transform(*x))
        .collect();
    let smallest = transformed.iter().min().unwrap();
    println!("Day 5 Part 1: {smallest}");

    let mut global_min: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(None));
    let chunked_seeds: Vec<&[u64]> = almanac.seeds.par_chunks(2).collect();
    chunked_seeds.par_iter().for_each(|seeds| {
        let mut range: Vec<u64> = (seeds[0]..(seeds[0] + seeds[1])).collect();
        println!("transforming slice");
        almanac.transform_slice(&mut range);
        if let Some(min) = range.par_iter().min() {
            let global_min = global_min.clone();
            let mut global_min = global_min.lock().unwrap();
            if  global_min.is_none() || min < &global_min.unwrap() {
                println!("found min");
                *global_min = Some(*min);
            }
        }
    });
    // for seeds in almanac.seeds.chunks(2) {
    //     let mut range: Vec<u64> = (seeds[0]..(seeds[0] + seeds[1])).collect();
    //     dbg!(range.len());
    //     almanac.transform_slice(&mut range);
    //     if let Some(min) = range.par_iter().min() {
    //         if  global_min.is_none() || min < &global_min.unwrap() {
    //             global_min = Some(*min);
    //         }
    //     }
    // }
    println!("Day 5 Part 2: {}", global_min.lock().unwrap().unwrap());

    Ok(())
}
