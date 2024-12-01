use std::cmp::max;
use std::fs;
use std::iter::zip;

pub fn day01() {
    let input = "data/input-01.txt";
    let file = fs::read_to_string(input).unwrap();
    let (mut col1, mut col2) = (Vec::new(), Vec::new());

    // parse each line in the file
    for line in file.lines() {
        if let Some((val1, val2)) = line.split_once("   ") {
            col1.push(val1.parse::<i32>().unwrap());
            col2.push(val2.parse::<i32>().unwrap());
        }
    }

    // sort the vectors
    col1.sort();
    col2.sort();

    // compute the distance
    let dist: i32 = zip(&col1, &col2).map(|(v1, v2)| (v1 - v2).abs()).sum();
    println!("Part 1: {}", dist); // 2815556

    // largest value encountered
    let max = *max(col1.last(), col2.last()).unwrap() as usize;

    // counts for each value in col2
    let mut counts = vec![0; max + 1];
    col2.iter().for_each(|&val| counts[val as usize] += 1);

    // compute the similarity
    let sim: i32 = col1.iter().map(|&val| val * counts[val as usize]).sum();
    println!("Part 2: {}", sim); // 23927637
}
