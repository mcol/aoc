use std::collections::HashMap;
use std::fs;

pub fn day03() {
    let input = "data/input-03.txt";
    let file = fs::read_to_string(input)
        .unwrap()
        .replace('x', ",")
        .replace(": ", ",");
    let mut vals = Vec::new();
    for line in file.lines() {
        let line: Vec<_> = line
            .split("@ ")
            .last()
            .unwrap()
            .split(',')
            .map(|z| z.parse::<i32>().unwrap())
            .collect();
        vals.push(line);
    }

    let mut patterns = HashMap::new();
    for val in &vals {
        if let [lef, top, wid, hei] = val[..] {
            for i in lef..(lef + wid) {
                for j in top..(top + hei) {
                    *patterns.entry((i, j)).or_insert(0) += 1;
                }
            }
        }
    }
    let res = patterns.values().fold(0, |acc, &z| acc + (z > 1) as i32);
    println!("Part 1: {res}");
    assert_eq!(res, 111326);

    let mut res = 0;
    'outer: for (idx, val) in vals.iter().enumerate() {
        if let [lef, top, wid, hei] = val[..] {
            for i in lef..(lef + wid) {
                for j in top..(top + hei) {
                    if patterns.get(&(i, j)) > Some(&1) {
                        continue 'outer;
                    }
                }
            }
        }
        res = idx + 1;
        break;
    }
    println!("Part 2: {res}");
    assert_eq!(res, 1019);
}
