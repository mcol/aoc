use std::fs;

pub fn day03() {
    let input = "data/input-03.txt";
    let file = fs::read_to_string(input).unwrap();

    fn parse_mul(line: &str) -> i32 {
        if let Some(idx_comma) = line.find(",") {
            let num1 = line[..idx_comma].parse::<i32>().unwrap_or_default();
            if num1 == 0 {
                return 0;
            }
            let idx0 = idx_comma + 1;
            if let Some(idx_paren) = line[idx0..].find(")") {
                let num2 = line[idx0..idx0 + idx_paren]
                    .parse::<i32>()
                    .unwrap_or_default();
                return num1 * num2;
            }
        }
        0
    }

    // parse each line in the file
    let mut tot = 0;
    for line in file.lines() {
        for (idx_mul, _) in line.match_indices("mul(") {
            tot += parse_mul(&line[idx_mul + 4..]);
        }
    }

    println!("Part 1: {}", tot); // 191183308

    let (mut is_on, mut tot) = (true, 0);
    for line in file.lines() {
        let on: Vec<_> = line
            .match_indices("do()")
            .map(|(idx, _)| (idx + 4, true))
            .collect();
        let off: Vec<_> = line
            .match_indices("don't()")
            .map(|(idx, _)| (idx, false))
            .collect();
        let mut onoff = [on, off].concat();
        onoff.sort();

        // store what is the last state on this line
        let last_state = onoff.last().unwrap().1;

        // we add another off state to evaluate the line
        onoff = [onoff, vec![(line.len(), false)]].concat();

        // if the last state was on, we need to turn it back on
        if last_state {
            onoff = [onoff, vec![(line.len(), true)]].concat();
        }

        let mut beg_idx = 0;
        for val in onoff.iter() {
            // same state that we are currently in
            if is_on == val.1 {
                continue;
            }

            // switch off
            if !val.1 {
                let this = &line[beg_idx..val.0];
                for (idx_mul, _) in this.match_indices("mul(") {
                    tot += parse_mul(&this[idx_mul + 4..]);
                }
                is_on = false;
            }
            // switch on
            else {
                beg_idx = val.0;
                is_on = true;
            }
        }
    }

    println!("Part 2: {}", tot); // 92082041
}
