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
    while dx <= x_tgt.start {
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
        .replace("target area: x=", "")
        .replace(", y=", "|")
        .replace("\n", "");
    let split: Vec<_> = file.split("|").collect();
    let x_tgt: Vec<_> = split[0]
        .split("..")
        .map(|z| z.parse::<i32>().unwrap())
        .collect();
    let y_tgt: Vec<_> = split[1]
        .split("..")
        .map(|z| z.parse::<i32>().unwrap())
        .collect();
    let x_tgt = x_tgt[0]..(x_tgt[1] + 1);
    let y_tgt = y_tgt[0]..(y_tgt[1] + 1);

    let mut max_dy = 0;
    for dx in velocity_xrange(&x_tgt) {
        for dy in max_dy..1000 {
            if hit((dx, dy), &x_tgt, &y_tgt) && dy > max_dy {
                max_dy = dy;
            }
        }
    }
    println!("Part 1: {}", max_dy * (max_dy + 1) / 2);

    let mut count = 0;
    for dx in velocity_xrange(&x_tgt).start..x_tgt.end {
        for dy in -1000..1000 {
            if hit((dx, dy), &x_tgt, &y_tgt) {
                count += 1;
            }
        }
    }
    println!("Part 2: {}", count);
}
