use std::fs;

pub fn day07() {
    let input = "data/input-07.txt";
    let crabs: Vec<_> = fs::read_to_string(input)
        .unwrap()
        .trim()
        .split(',')
        .map(|z| z.to_string().parse::<i32>().unwrap())
        .collect();
    let (min, max) = (
        *crabs.iter().min().unwrap() as usize,
        *crabs.iter().max().unwrap() as usize,
    );

    let min_cost = (min..=max).fold(i32::MAX, |acc, pos| {
        let cost = crabs.iter().map(|z| i32::abs(*z - pos as i32)).sum();
        i32::min(acc, cost)
    });
    println!("Part 1: {min_cost}");
    assert_eq!(min_cost, 352707);

    let mut min_cost = u32::MAX;
    for pos in min..=max {
        let cost = crabs
            .iter()
            .map(|z| {
                let c = z.abs_diff(pos as i32);
                c * (c + 1) / 2
            })
            .sum();
        min_cost = u32::min(cost, min_cost);
    }
    println!("Part 2: {min_cost}");
    assert_eq!(min_cost, 95519693);
}
