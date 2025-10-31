use std::fs;

fn power_level(serialno: usize, grid_size: usize) -> Vec<Vec<isize>> {
    let mut power = vec![vec![0; grid_size]; grid_size];
    for (x_idx, x_val) in power.iter_mut().enumerate() {
        let rack_id = x_idx + 10 + 1;
        for (y_idx, y_val) in x_val.iter_mut().enumerate() {
            let pow = (rack_id * (y_idx + 1) + serialno) * rack_id;
            *y_val = ((pow % 1000) / 100) as isize - 5;
        }
    }
    power
}

fn sum_square(pows: &[Vec<isize>], x_idx: usize, size: usize) -> isize {
    pows.iter()
        .skip(x_idx)
        .take(size)
        .fold(0, |acc, v| acc + v.iter().take(size).sum::<isize>())
}

fn find_square(pows: &[Vec<isize>], size: usize) -> (isize, usize, usize) {
    let last_idx = pows.len() - size + 1;
    let (mut max_sum, mut x, mut y) = (0, 0, 0);
    for row in 0..last_idx {
        let mut sum: isize = sum_square(pows, row, size);
        for col in 1..last_idx {
            sum += pows
                .iter()
                .skip(row)
                .take(size)
                .fold(0, |acc, v| acc + v[col + size - 1] - v[col - 1]);
            if sum > max_sum {
                (max_sum, x, y) = (sum, row + 1, col + 1);
            }
        }
    }
    (max_sum, x, y)
}

pub fn day11() {
    let input = "data/input-11.txt";
    let serialno = fs::read_to_string(input)
        .unwrap()
        .replace('\n', "")
        .parse::<usize>()
        .unwrap();
    let grid_size = 300;
    let pows = power_level(serialno, grid_size);

    let (_, x, y) = find_square(&pows, 3);
    let res = format!("{x},{y}");
    println!("Part 1: {res}");
    assert_eq!(res, "243,16");

    let max_squares: Vec<_> = (1..grid_size)
        .map(|size| find_square(&pows, size))
        .collect();
    let res = max_squares
        .iter()
        .enumerate()
        .max_by_key(|&(_, &(sum, _, _))| sum)
        .map(|(idx, (_, x, y))| format!("{x},{y},{}", idx + 1))
        .unwrap();
    println!("Part 2: {res}");
    assert_eq!(res, "231,227,14");
}
