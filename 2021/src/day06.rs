use std::fs;

pub fn day06() {
    let input = "data/input-06.txt";
    let mut vals: Vec<_> = fs::read_to_string(input)
        .unwrap()
        .trim()
        .split(',')
        .map(|z| z.to_string().parse::<i32>().unwrap())
        .collect();
    let orig = vals.clone();

    for _day in 1..=80 {
        let mut new_vals = Vec::new();
        for val in &mut vals {
            if *val == 0 {
                *val = 7;
                new_vals.push(8i32);
            }
            *val -= 1;
        }
        vals.append(&mut new_vals);
    }
    let res = vals.len();
    println!("Part 1: {res}");
    assert_eq!(res, 365862);

    let mut states = [0u64; 9];
    for v in orig {
        states[v as usize] += 1;
    }
    for _day in 1..=256 {
        let s0 = states[0];
        for state in 1..=8 {
            states[state - 1] = states[state];
        }
        states[8] = s0;
        states[6] += s0;
    }
    let res = states.iter().sum::<u64>();
    println!("Part 2: {res}");
    assert_eq!(res, 1653250886439);
}
