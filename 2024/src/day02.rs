use std::fs;

pub fn day02() {
    let input = "data/input-02.txt";
    let file = fs::read_to_string(input).unwrap();
    let mut safe = vec![true; file.lines().count()];

    // check the difference between successive elements
    fn eval_safe(levs: &[i32]) -> bool {
        for lev in levs[..].windows(2) {
            let diff = lev[1] - lev[0];
            if !(1..=3).contains(&diff) {
                return false;
            }
        }
        true
    }

    // parse each line in the file
    for (idx, line) in file.lines().enumerate() {
        let mut levs = line
            .split(' ')
            .map(|it| it.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // reverse the array if decreasing
        if levs[1] < levs[0] {
            levs.reverse();
        }
        safe[idx] = eval_safe(&levs);
    }

    // count the safe lines
    let num = safe.iter().filter(|&x| *x).count();
    println!("Part 1: {}", num); // 326

    // parse each line in the file
    let mut safe = vec![false; file.lines().count()];
    for (idx, line) in file.lines().enumerate() {
        let mut levs = line
            .split(' ')
            .map(|it| it.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // we need to check the first 3 diffs before we can decide if the
        // array is decreasing
        let mut num_decr = 0;
        for i in 1..4 {
            if levs[i] < levs[i - 1] {
                num_decr += 1
            }
        }

        // reverse the array if decreasing
        if num_decr > 1 {
            levs.reverse()
        }

        // check if safe without requiring exceptions
        if eval_safe(&levs) {
            safe[idx] = true;
            continue;
        }

        // check if it is safe with one exception
        for i in 0..levs.len() {
            let mut sub = levs.clone();
            sub.remove(i);
            if eval_safe(&sub) {
                safe[idx] = true;
                break;
            }
        }
    }

    // count the safe lines
    let num = safe.iter().filter(|&x| *x).count();
    println!("Part 2: {}", num); // 381
}
