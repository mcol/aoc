use std::collections::HashSet;
use std::fs;

pub fn day01() {
    let input = "data/input-01.txt";
    let file = fs::read_to_string(input).unwrap();
    let vals: Vec<_> = file.lines().map(|z| z.parse::<i32>().unwrap()).collect();

    let sum: i32 = vals.iter().sum();
    println!("Part 1: {sum}");
    assert_eq!(sum, 587);

    let (mut seen, mut sum) = (HashSet::new(), 0);
    'outer: loop {
        for val in &vals {
            sum += val;
            if !seen.insert(sum) {
                break 'outer;
            }
        }
    }
    println!("Part 2: {sum}");
    assert_eq!(sum, 83130);
}
