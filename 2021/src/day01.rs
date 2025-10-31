use std::fs;

pub fn day01() {
    let input = "data/input-01.txt";
    let file = fs::read_to_string(input).unwrap();
    let file = file.split("\n");
    let mut vals = Vec::with_capacity(file.clone().count());

    for token in file.clone() {
        match token.parse::<i32>() {
            Ok(num) => vals.push(num),
            Err(_) => continue,
        };
    }

    let mut last = vals[0];
    let mut incr = 0;
    for curr in vals.clone() {
        if curr > last {
            incr += 1;
        }
        last = curr;
    }
    println!("Part 1: {}", incr);

    let mut incr = 0;
    for idx in 3..vals.len() {
        if vals[idx] > vals[idx - 3] {
            incr += 1;
        }
    }
    println!("Part 2: {}", incr);
}
