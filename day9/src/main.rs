use std::fs::read_to_string;

fn read_input() -> Vec<Vec<i32>>{
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

fn predict(nums: &[i32]) -> i32 {
    let mut diffs = vec![];
    for n in nums.windows(2) {
        let diff = n[1] - n[0];
        diffs.push(diff);
    }
    if nums.iter().all(|x| *x == 0) {
        return 0;
    } else {
        return predict(&diffs) + nums.iter().last().unwrap();
    }
}
fn main() {
    let hists = read_input();
    let sum: i32 = hists.iter().map(|x| predict(x)).sum();
    println!("Day 9 Part 1: {sum}");
}
