use std::fs;

pub fn day07() {
    let input = "data/input-07.txt";
    let file = fs::read_to_string(input).unwrap().replace("\n", "");

    let mut crabs = Vec::new();
    for token in file.split(',') {
        crabs.push(token.to_string().parse::<i32>().expect(""));
    }

    let (min, max) = (
        *crabs.iter().min().unwrap() as usize,
        *crabs.iter().max().unwrap() as usize,
    );

    let mut min_cost = i32::MAX;
    for pos in min..=max {
        let cost = crabs.iter().map(|z| i32::abs(*z - pos as i32)).sum();
        min_cost = i32::min(cost, min_cost);
    }
    println!("part 1: {}", min_cost);

    let mut min_cost = i32::MAX;
    for pos in min..=max {
        let cost = crabs
            .iter()
            .map(|z| {
                let c = i32::abs(*z - pos as i32);
                c * (c + 1) / 2
            })
            .sum();
        min_cost = i32::min(cost, min_cost);
    }
    println!("part 2: {}", min_cost);
}
