use std::fs;

pub fn day02() {
    let input = "data/input-02.txt";
    let file = fs::read_to_string(input).unwrap();

    let (mut horiz, mut depth) = (0, 0);
    for line in file.lines() {
        let mut s = line.split_whitespace();
        let dir = s.next();
        let num = s.next();
        let val = match num {
            Some(n) => n.parse::<i32>().expect(""),
            None => continue,
        };
        match dir {
            Some("forward") => horiz += val,
            Some("up") => depth -= val,
            Some("down") => depth += val,
            Some(_) => continue,
            None => continue,
        };
    }
    println!("Part 1: {}", horiz * depth);

    let file = fs::read_to_string(input).unwrap();
    let (mut horiz, mut depth, mut aim) = (0, 0, 0);
    for line in file.lines() {
        let mut s = line.split_whitespace();
        let dir = s.next();
        let num = s.next();
        let val = match num {
            Some(n) => n.parse::<i32>().expect(""),
            None => continue,
        };
        match dir {
            Some("forward") => {
                horiz += val;
                depth += val * aim;
            }
            Some("up") => aim -= val,
            Some("down") => aim += val,
            Some(_) => continue,
            None => continue,
        };
    }
    println!("Part 2: {}", horiz * depth);
}
