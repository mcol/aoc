use std::fs;

pub fn day01() {
    let input = "data/input-01.txt";
    let file = fs::read_to_string(input).unwrap();
    let vals: Vec<_> = file.lines().map(|z| z.parse::<i32>().unwrap()).collect();
    let slide_window = |x: &[i32], size| {
        x.iter()
            .enumerate()
            .skip(size)
            .fold(0, |acc, (idx, &val)| acc + (val > vals[idx - size]) as i32)
    };

    let res = slide_window(&vals, 1);
    println!("Part 1: {res}");
    assert_eq!(res, 1557);

    let res = slide_window(&vals, 3);
    println!("Part 2: {res}");
    assert_eq!(res, 1608);
}
