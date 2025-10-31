use std::collections::HashSet;
use std::fs;

pub fn day03() {
    let input = "data/input-03.txt";
    let size = if input.contains("example") { 5 } else { 12 };

    // read the contents of the file
    let file = fs::read_to_string(input).unwrap();
    let mut vecs = Vec::new();
    for line in file.lines() {
        let vals: Vec<u32> = line
            .chars()
            .collect::<Vec<char>>()
            .into_iter()
            .map(|z| z.to_digit(10).unwrap())
            .collect();
        vecs.push(vals);
    }

    let mut sums = vec![0; size];
    for vals in &vecs {
        for idx in 0..size {
            sums[idx] += vals[idx];
        }
    }

    let (mut gamma, mut epsilon) = (0, 0);
    let half = vecs.len() as u32 / 2;
    for (idx, &val) in sums.iter().enumerate() {
        if val > half {
            gamma += i32::pow(2, (size - 1 - idx) as u32);
        } else {
            epsilon += i32::pow(2, (size - 1 - idx) as u32);
        }
    }
    println!("Part 1: {}", gamma * epsilon);

    let oxy = find_oxy_co2(&vecs, size, true);
    let co2 = find_oxy_co2(&vecs, size, false);
    println!("Part 2: {}", oxy * co2);
}

fn find_oxy_co2(vecs: &[Vec<u32>], size: usize, is_oxy: bool) -> i32 {
    let mut keep = HashSet::new();
    for idx in 0..vecs.len() {
        keep.insert(idx);
    }

    for idx in 0..size {
        let mut sum = 0;
        for (num, val) in vecs.iter().enumerate() {
            if keep.contains(&num) {
                sum += val[idx];
            }
        }
        let val = if sum as f32 >= keep.len() as f32 / 2. {
            is_oxy as u32
        } else {
            !is_oxy as u32
        };

        for (num, vals) in vecs.iter().enumerate() {
            if keep.contains(&num) && vals[idx] != val {
                keep.remove(&num);
            }
        }
        if keep.len() == 1 {
            break;
        }
    }

    let idx: usize = *keep.iter().collect::<Vec<&usize>>()[0];
    let val = &vecs[idx];
    let mut res = 0;
    for (idx, &val) in val.iter().enumerate() {
        if val == 1 {
            res += i32::pow(2, (size - 1 - idx) as u32);
        }
    }
    res
}
