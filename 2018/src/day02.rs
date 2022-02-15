use std::fs;

pub fn day02() {
    let input = "data/input-02.txt";
    let file = fs::read_to_string(input).unwrap();
    let vals: Vec<_> = file.lines().collect();

    let (mut num2, mut num3) = (0, 0);
    for val in &vals {
        let counts: Vec<_> = val.chars().map(|z| val.matches(z).count()).collect();
        if counts.contains(&2) {
            num2 += 1;
        }
        if counts.contains(&3) {
            num3 += 1;
        }
    }
    let res = num2 * num3;
    println!("Part 1: {res}");
    assert_eq!(res, 6200);

    let mut res = String::new();
    'outer: for (idx, val1) in vals.iter().enumerate() {
        for val2 in &vals[(idx + 1)..] {
            res = val1
                .chars()
                .zip(val2.chars())
                .filter_map(|(x, y)| if x == y { Some(x) } else { None })
                .collect();
            if val1.len() - res.len() == 1 {
                break 'outer;
            }
        }
    }
    println!("Part 2: {res}");
    assert_eq!(res, "xpysnnkqrbuhefmcajodplyzw");
}
