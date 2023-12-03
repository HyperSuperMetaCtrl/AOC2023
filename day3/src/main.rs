use anyhow::Result;
use ndarray::{s, Array};
use ndarray_ndimage::{pad, PadMode};
use std::{fs::read_to_string, collections::HashMap};

const DIM: usize = 140;
type Point = (usize, usize);
struct Screw {
    touching: usize,
    value: usize,
}

fn main() -> Result<()> {
    let mut sum = 0;
    let input = read_to_string("input.txt")?;
    let input = input.replace("\n", "");
    let mat = Array::from_shape_vec((DIM, DIM), input.into_bytes())?;
    let mat = pad(&mat, &[[1usize, 1usize]], PadMode::Constant(b'.'));
    let mut skip = 0;
    let mut screws: HashMap<Point, Screw>= HashMap::new();
    for (index, elem) in mat.indexed_iter() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        if elem.is_ascii_digit() {
            let start = index.1;
            let mut offset = 0;
            while mat[[index.0, start + offset + 1]].is_ascii_digit() {
                offset += 1;
            }
            let num = mat.slice(s!(index.0, start..=start + offset));
            let num = num.as_slice().unwrap();
            let num = u32::from_str_radix(std::str::from_utf8(num).unwrap(), 10).unwrap();

            // look for symbols
            let frame = mat.slice(s!(
                index.0 - 1..=index.0 + 1,
                start - 1..=start + offset + 1
            ));
            let count = frame
                .iter()
                .any(|b| !b.is_ascii_digit() && *b != b'.' && b.is_ascii_punctuation());
            if let Some(screw) = frame
                .iter()
                .position(|b| *b == b'*') {todo!()}
            if count == true {
                sum += num;
                skip = offset
            }
        }
    }
    println!("Day 3 Part 1: {sum}");
    Ok(())
}
