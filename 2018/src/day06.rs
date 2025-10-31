use std::collections::HashSet;
use std::fs;

fn manhattan(a: &[i32], b: &[i32]) -> i32 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
}

pub fn day06() {
    let input = "data/input-06.txt";
    let file = fs::read_to_string(input).unwrap();
    let mut coords = Vec::new();
    for line in file.lines() {
        let line: Vec<_> = line
            .split(", ")
            .map(|z| z.parse::<i32>().unwrap())
            .collect();
        coords.push(line);
    }
    let range = |acc: &[i32], val: i32| [i32::min(acc[0], val), i32::max(acc[1], val)];
    let [min_row, max_row] = coords
        .iter()
        .fold([i32::MAX, 0], |acc, z| range(&acc, z[0]));
    let [min_col, max_col] = coords
        .iter()
        .fold([i32::MAX, 0], |acc, z| range(&acc, z[1]));

    let (mut owner, mut region_size) = (vec![0; coords.len()], 0);
    let mut infinite = HashSet::new();
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            let dist: Vec<_> = coords.iter().map(|z| manhattan(&[row, col], z)).collect();
            let min_dist = dist.iter().min().unwrap();
            let min_idx: Vec<_> = dist
                .iter()
                .enumerate()
                .filter(|&(_, z)| z == min_dist)
                .map(|(idx, _)| idx)
                .collect();
            if min_idx.len() == 1 {
                owner[min_idx[0]] += 1;
                if (row == min_row) || (row == max_row) || (col == min_col) || (col == max_col) {
                    infinite.insert(min_idx[0]);
                }
            }
            region_size += (dist.iter().sum::<i32>() < 10000) as i32;
        }
    }

    let res = owner.iter().enumerate().fold(0, |acc, (idx, &val)| {
        i32::max(acc, val * (!infinite.contains(&idx) as i32))
    });
    println!("Part 1: {res}");
    assert_eq!(res, 3907);

    let res = region_size;
    println!("Part 2: {res}");
    assert_eq!(res, 42036);
}
