use std::collections::HashSet;
use std::fs;

pub fn day09() {
    let input = "data/input-09.txt";
    let file = fs::read_to_string(input).unwrap();
    let vals: Vec<_> = file
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|z| z.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect();

    let (nrows, ncols) = (vals.len(), vals[0].len());
    let mut risk = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            let up = (i == 0) || vals[i - 1][j] > vals[i][j];
            let dn = match vals.get(i + 1) {
                Some(v) => v[j] > vals[i][j],
                None => true,
            };
            let lf = (j == 0) || vals[i][j - 1] > vals[i][j];
            let rt = match vals[i].get(j + 1) {
                Some(v) => *v > vals[i][j],
                None => true,
            };
            if up && dn && lf && rt {
                risk += vals[i][j] + 1;
            }
        }
    }
    println!("Part 1: {risk}");
    assert_eq!(risk, 417);

    let mut tovisit = HashSet::new();
    for i in 0..nrows {
        for j in 0..ncols {
            tovisit.insert((i, j));
        }
    }

    let (mut basins, mut queued) = (vec![0], HashSet::new());
    queued.insert((0, 0));
    while !tovisit.is_empty() {
        if let Some(&(i, j)) = queued.iter().last() {
            queued.remove(&(i, j));
            tovisit.remove(&(i, j));
            if vals[i][j] == 9 {
                continue;
            }

            let idx = basins.len();
            basins[idx - 1] += 1;
            if tovisit.contains(&(i + 1, j)) {
                queued.insert((i + 1, j));
            }
            if i > 0 && tovisit.contains(&(i - 1, j)) {
                queued.insert((i - 1, j));
            }
            if tovisit.contains(&(i, j + 1)) {
                queued.insert((i, j + 1));
            }
            if j > 0 && tovisit.contains(&(i, j - 1)) {
                queued.insert((i, j - 1));
            }
        } else {
            queued.insert(*tovisit.iter().take(1).next().unwrap());
            basins.push(0);
        }
    }

    basins.sort_unstable();
    let res: i32 = basins.iter().rev().take(3).product();
    println!("Part 2: {res}");
    assert_eq!(res, 1148965);
}
