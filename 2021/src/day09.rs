use std::collections::HashSet;
use std::fs;

pub fn day09() {
    let input = "data/input-09.txt";
    let _input = "data/example-09.txt";
    let file = fs::read_to_string(input).unwrap();

    let mut vals = Vec::new();
    for line in file.lines() {
        let digits: Vec<u32> = line.chars().map(|z| z.to_digit(10).unwrap()).collect();
        vals.push(digits);
    }

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
            if up & dn & lf & rt {
                risk += vals[i][j] + 1;
            }
        }
    }
    println!("Part 1: {}", risk);

    let mut queued = HashSet::new();
    let mut basins = vec![0];
    let mut visited = vec![vec![0; ncols]; nrows];
    let mut tovisit = HashSet::new();
    for i in 0..nrows {
        for j in 0..ncols {
            tovisit.insert((i, j));
        }
    }

    queued.insert((0, 0));
    while !tovisit.is_empty() {
        if let Some(&(i, j)) = queued.iter().last() {
            queued.remove(&(i, j));
            tovisit.remove(&(i, j));
            if vals[i][j] == 9 {
                visited[i][j] = 999;
                continue;
            }

            let idx = basins.len();
            visited[i][j] = idx;
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
    let prod: i32 = basins.iter().rev().take(3).product();
    println!("Part 2: {}", prod);
}
