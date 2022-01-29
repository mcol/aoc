use std::collections::HashSet;
use std::fs;

pub fn day03() {
    let input = "data/input-03.txt";
    let size = if input.contains("example") { 5 } else { 12 };
    let file = fs::read_to_string(input).unwrap();
    let vecs: Vec<_> = file
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|z| z.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect();
    let sums: Vec<_> = vecs.iter().fold(vec![0; size], |acc, z| {
        acc.iter().zip(z.iter()).map(|(v1, v2)| v1 + v2).collect()
    });

    let (mut gamma, mut epsilon) = (0, 0);
    let half = vecs.len() as u32 / 2;
    for (idx, &sum) in sums.iter().enumerate() {
        if sum > half {
            gamma += i32::pow(2, (size - 1 - idx) as u32);
        } else {
            epsilon += i32::pow(2, (size - 1 - idx) as u32);
        }
    }
    let res = gamma * epsilon;
    println!("Part 1: {res}");
    assert_eq!(res, 2972336);

    let oxy = find_oxy_co2(&vecs, size, true);
    let co2 = find_oxy_co2(&vecs, size, false);
    let res = oxy * co2;
    println!("Part 2: {res}");
    assert_eq!(res, 3368358);
}

fn find_oxy_co2(vecs: &[Vec<u32>], size: usize, is_oxy: bool) -> i32 {
    let mut keep: HashSet<_> = (0..vecs.len()).collect();
    for idx in 0..size {
        let mut sum = 0;
        for (num, vals) in vecs.iter().enumerate() {
            if keep.contains(&num) {
                sum += vals[idx];
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

    let idx = keep.drain().collect::<Vec<_>>()[0];
    vecs[idx]
        .iter()
        .enumerate()
        .map(|(idx, &val)| i32::pow(2, (size - 1 - idx) as u32) * val as i32)
        .sum()
}
