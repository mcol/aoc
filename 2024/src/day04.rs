use std::fs;

pub fn check_match(chars: &[char], idx: isize, offset: isize) -> bool {
    let max_idx = idx + 3 * offset;
    max_idx >= 0
        && max_idx < chars.len() as isize
        && 'M' == chars[(idx + offset) as usize]
        && 'A' == chars[(idx + 2 * offset) as usize]
        && 'S' == chars[(idx + 3 * offset) as usize]
}

pub fn check_cross(chars: &[char], idx: isize, offset1: isize, offset2: isize) -> bool {
    if idx + offset2 < 0
        || idx - offset2 < 0
        || idx + offset2 >= chars.len() as isize
        || idx - offset2 >= chars.len() as isize
    {
        return false;
    }

    // all possible endpoints of the cross
    let mut slice = [
        chars[(idx + offset1) as usize],
        chars[(idx - offset1) as usize],
        chars[(idx + offset2) as usize],
        chars[(idx - offset2) as usize],
    ];

    // valid endpoints are only 'M' or 'S'
    slice.sort();
    if slice[0] != 'M' || slice[3] != 'S' {
        return false;
    }

    // check that the endpoints are different
    chars[(idx + offset1) as usize] != chars[(idx - offset1) as usize]
        && chars[(idx + offset2) as usize] != chars[(idx - offset2) as usize]
}

pub fn day04() {
    let input = "data/input-04.txt";
    let file = fs::read_to_string(input).unwrap();
    let rows = file.lines().count();

    // extract all chars in the file
    let chars: Vec<_> = file.chars().collect();
    let ncols = (chars.len() / rows) as isize;

    let mut num = 0;
    for (idx, &val) in chars.iter().enumerate() {
        if val == 'X' {
            for offset in [-1, -1 + ncols, ncols, 1 + ncols] {
                if check_match(&chars, idx as isize, offset) {
                    num += 1;
                }
                if check_match(&chars, idx as isize, -offset) {
                    num += 1;
                }
            }
        }
    }
    println!("Part 1: {}", num); // 2378

    let num = chars
        .iter()
        .enumerate()
        .map(|(idx, &val)| val == 'A' && check_cross(&chars, idx as isize, 1 - ncols, 1 + ncols))
        .filter(|&x| x)
        .count();
    println!("Part 2: {}", num); // 1796
}
