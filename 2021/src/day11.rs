use std::collections::HashSet;
use std::fs;

fn neighbours(nrows: usize, ncols: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    if i > 0 {
        v.push((i - 1, j));
        if j > 0 {
            v.push((i - 1, j - 1));
        }
        if j < ncols - 1 {
            v.push((i - 1, j + 1));
        }
    }
    if i < nrows - 1 {
        v.push((i + 1, j));
        if j > 0 {
            v.push((i + 1, j - 1));
        }
        if j < ncols - 1 {
            v.push((i + 1, j + 1));
        }
    }
    if j > 0 {
        v.push((i, j - 1));
    }
    if j < ncols - 1 {
        v.push((i, j + 1));
    }
    v
}

pub fn day11() {
    let input = "data/input-11.txt";
    let file = fs::read_to_string(input).unwrap();
    let mut vals: Vec<_> = file
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|z| z.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect();

    let (nrows, ncols) = (vals.len(), vals[0].len());
    let mut exploding = HashSet::new();
    let (mut flashed, mut step) = (0, 0);
    loop {
        step += 1;
        for (i, row) in vals.iter_mut().enumerate().take(nrows) {
            for (j, val) in row.iter_mut().enumerate().take(ncols) {
                *val += 1;
                if *val == 10 {
                    exploding.insert((i, j));
                }
            }
        }
        while !exploding.is_empty() {
            if let Some(&(i, j)) = exploding.iter().last() {
                flashed += 1;
                exploding.remove(&(i, j));
                for (i_, j_) in neighbours(nrows, ncols, i, j) {
                    vals[i_][j_] += 1;
                    if vals[i_][j_] == 10 {
                        exploding.insert((i_, j_));
                    }
                }
            }
        }

        let mut flashing = 0;
        for row in vals.iter_mut().take(nrows) {
            for val in row.iter_mut().take(ncols) {
                if *val > 9 {
                    *val = 0;
                    flashing += 1;
                }
            }
        }
        if step == 100 {
            println!("Part 1: {flashed}");
            assert_eq!(flashed, 1661);
        }
        if flashing == nrows * ncols {
            println!("Part 2: {step}");
            assert_eq!(step, 334);
            break;
        }
    }
}
