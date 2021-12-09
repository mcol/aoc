use std::fs;

pub fn day06() {
    let input = "data/input-06.txt";
    let file = fs::read_to_string(input).unwrap().replace("\n", "");

    let mut vals = Vec::new();
    for token in file.split(',') {
        vals.push(token.to_string().parse::<i32>().expect(""));
    }
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
    println!("Part 1: {}", vals.len());

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
    println!("Part 2: {:?}", states.iter().sum::<u64>());
}
