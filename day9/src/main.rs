use std::fs::read_to_string;

fn read_input() -> Vec<Vec<i32>> {
    let input = read_to_string("input.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();
    let mut hists = Vec::with_capacity(lines.len());
    for line in lines {
        let nums: Vec<_> = line.split(" ").collect();
        let nums: Vec<_> = nums
            .into_iter()
            .map(|x| str::parse::<i32>(x).unwrap())
            .collect();
        hists.push(nums);
    }
    hists
}

fn diffs(nums: &[i32]) -> Vec<i32> {
    let mut diffs = Vec::with_capacity(nums.len()-1);
    for n in nums.windows(2) {
        let diff = n[1] - n[0];
        diffs.push(diff);
    }
    diffs
}
enum Direction {
    L,
    R,
}

fn predict(nums: &[i32], direction: Direction) -> i32 {
    let diffs = diffs(nums);
    if nums.iter().all(|x| *x == 0) {
        return 0;
    }
    match direction {
        Direction::L => nums[0] - predict(&diffs, Direction::L),
        Direction::R => predict(&diffs, Direction::R) + nums[nums.len() - 1],
    }
}

fn main() {
    let hists = read_input();
    let predictions: Vec<(i32, i32)> = hists
        .iter()
        .map(|x| (predict(x, Direction::L), predict(x, Direction::R)))
        .collect();
    let (part2, part1): (i32, i32) = predictions
        .iter()
        .fold((0, 0), |acc, (x, y)| (acc.0 + x, acc.1 + y));
    println!("Day 9 Part 1: {part1}, Part 2: {part2}");
}
