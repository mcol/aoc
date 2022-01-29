use std::fs;
use std::ops::Range;

fn hit(vel: (i32, i32), x_tgt: &Range<i32>, y_tgt: &Range<i32>) -> bool {
    let (mut x, mut y, (mut dx, mut dy)) = (0, 0, vel);
    while x <= x_tgt.end && y >= y_tgt.start {
        x += dx;
        y += dy;
        if x_tgt.contains(&x) && y_tgt.contains(&y) {
            return true;
        }
        dx = i32::max(dx - 1, 0);
        dy -= 1;
    }
    false
}
fn velocity_xrange(x_tgt: &Range<i32>) -> Range<i32> {
    // the terminal position x, the one at which the probe drops vertically,
    // corresponds to dx * (dx + 1) / 2, so we can derive the minimum value
    // by solving for dx
    let dx_min = -1. + (f32::sqrt(1. + 8. * x_tgt.start as f32)) * 0.5;
    let dx_min = f32::ceil(dx_min) as i32;
    let (mut dx, mut dx_max) = (dx_min, 0);
    while dx <= x_tgt.start as i32 {
        if x_tgt.contains(&(dx * (dx + 1) / 2)) {
            dx_max = dx;
        }
        dx += 1
    }
    dx_min..dx_max
}

pub fn day17() {
    let input = "data/input-17.txt";
    let file: String = fs::read_to_string(input)
        .unwrap()
        .trim()
        .replace("target area: x=", "")
        .replace(", y=", "|");
    let (x_tgt, y_tgt) = file.split_once('|').unwrap();
    let to_range = |range: &str| {
        let parse_i32 = |v: &str| v.parse::<i32>().unwrap();
        range
            .split_once("..")
            .map(|(v1, v2)| parse_i32(v1)..(parse_i32(v2) + 1))
            .unwrap()
    };
    let x_tgt = to_range(x_tgt);
    let y_tgt = to_range(y_tgt);

    let mut max_dy = 0;
    for dx in velocity_xrange(&x_tgt) {
        for dy in max_dy..1000 {
            if hit((dx, dy), &x_tgt, &y_tgt) && dy > max_dy {
                max_dy = dy;
            }
        }
    }
    let res = max_dy * (max_dy + 1) / 2;
    println!("Part 1: {res}");
    assert_eq!(res, 4278);

    let mut count = 0;
    for dx in velocity_xrange(&x_tgt).start..x_tgt.end {
        for dy in -1000..1000 {
            if hit((dx, dy), &x_tgt, &y_tgt) {
                count += 1;
            }
        }
    }
    println!("Part 2: {count}");
    assert_eq!(count, 1994);
}
