use std::fs;

pub fn day02() {
    let input = "data/input-02.txt";
    let file = fs::read_to_string(input).unwrap();

    let (mut horiz, mut depth) = (0, 0);
    for line in file.lines() {
        if let Some((dir, num)) = line.split_once(' ') {
            let val = num.parse::<i32>().unwrap();
            match dir {
                "forward" => horiz += val,
                "up" => depth -= val,
                "down" => depth += val,
                _ => panic!(),
            };
        }
    }
    let res = horiz * depth;
    println!("Part 1: {res}");
    assert_eq!(res, 1893605);

    let (mut horiz, mut depth, mut aim) = (0, 0, 0);
    for line in file.lines() {
        if let Some((dir, num)) = line.split_once(' ') {
            let val = num.parse::<i32>().unwrap();
            match dir {
                "forward" => {
                    horiz += val;
                    depth += val * aim;
                }
                "up" => aim -= val,
                "down" => aim += val,
                _ => panic!(),
            };
        }
    }
    let res = horiz * depth;
    println!("Part 2: {res}");
    assert_eq!(res, 2120734350);
}
