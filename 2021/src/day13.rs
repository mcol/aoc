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
        let split: Vec<&str> = line.split(',').collect();
        match split.len() {
            2 => {
                let x = split[0].parse::<i32>().unwrap();
                let y = split[1].parse::<i32>().unwrap();
                coords.insert((x, y));
            }
            _ => {
                let split: Vec<&str> = line.split('=').collect();
                if split.len() == 2 {
                    let axis = split[0][11..].to_owned();
                    let value = split[1].parse::<i32>().unwrap();
                    folds.push((axis, value));
                }
            }
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
            _ => panic!("unreachable"),
        };
        if !first_fold_done {
            println!("Part 1: {}", coords.len());
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
