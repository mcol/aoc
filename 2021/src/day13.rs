use std::collections::HashSet;
use std::fs;

fn fold(coords: HashSet<(i32, i32)>, along_x: bool, fold: i32) -> HashSet<(i32, i32)> {
    let mut folded = HashSet::new();
    for (mut x, mut y) in coords {
        if along_x && x > fold {
            x = 2 * fold - x;
        } else if !along_x && y > fold {
            y = 2 * fold - y;
        }
        folded.insert((x, y));
    }
    folded
}

pub fn day13() {
    let input = "data/input-13.txt";
    let file = fs::read_to_string(input).unwrap();
    let (mut coords, mut folds) = (HashSet::new(), Vec::new());
    for line in file.lines() {
        if let Some((x, y)) = line.split_once(',') {
            coords.insert((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
        } else if let Some((axis, value)) = line.split_once('=') {
            folds.push((axis[11..].to_owned(), value.parse::<i32>().unwrap()));
        }
    }

    let (mut rows, mut cols, mut first_fold_done) = (0, 0, false);
    for (axis, value) in folds {
        match axis.as_str() {
            "x" => {
                coords = fold(coords, true, value);
                cols = value;
            }
            "y" => {
                coords = fold(coords, false, value);
                rows = value;
            }
            _ => unreachable!(),
        };
        if !first_fold_done {
            let res = coords.len();
            println!("Part 1: {res}");
            assert_eq!(res, 720);
            first_fold_done = true;
        }
    }

    println!("Part 2: ");
    for i in 0..rows {
        for j in 0..cols {
            if coords.contains(&(j, i)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
