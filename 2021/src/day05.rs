use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(vals: Vec<&str>) -> Self {
        Point {
            x: vals[0].to_string().parse::<i32>().expect(""),
            y: vals[1].to_string().parse::<i32>().expect(""),
        }
    }
}

struct SteppedRange {
    x_curr: i32,
    x_targ: i32,
    x_step: i32,
    y_curr: i32,
    y_targ: i32,
    y_step: i32,
}

impl SteppedRange {
    fn new(src: Point, dst: Point) -> Self {
        let x_step = match dst.x.cmp(&src.x) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        let y_step = match dst.y.cmp(&src.y) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        SteppedRange {
            x_curr: src.x,
            y_curr: src.y,
            x_targ: dst.x + x_step,
            y_targ: dst.y + y_step,
            x_step,
            y_step,
        }
    }
}

impl Iterator for SteppedRange {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<(i32, i32)> {
        if (self.x_curr == self.x_targ) & (self.y_curr == self.y_targ) {
            None
        } else {
            let res = (self.x_curr, self.y_curr);
            self.x_curr += self.x_step;
            self.y_curr += self.y_step;
            Some(res)
        }
    }
}

fn points_in_line(pair: (Point, Point)) -> Vec<Point> {
    let (src, dst) = pair;
    let mut points = Vec::new();
    for (xx, yy) in SteppedRange::new(src, dst) {
        points.push(Point { x: xx, y: yy });
    }
    points
}

fn is_horizontal_or_vertical(pair: (Point, Point)) -> bool {
    let (src, dst) = pair;
    (src.x == dst.x) | (src.y == dst.y)
}

fn is_diagonal(pair: (Point, Point)) -> bool {
    let (src, dst) = pair;
    i32::abs(src.x - dst.x) == i32::abs(src.y - dst.y)
}

pub fn day05() {
    let input = "data/input-05.txt";
    let file = fs::read_to_string(input).unwrap();

    let (mut src, mut dst) = (Vec::new(), Vec::new());
    for line in file.lines() {
        let mut first = true;
        for token in line.split(" -> ") {
            let ppp = Point::new(token.split(',').collect::<Vec<&str>>());
            if first {
                src.push(ppp);
                first = false;
            } else {
                dst.push(ppp);
            }
        }
    }

    let (mut cover_p1, mut cover_p2) = (HashMap::new(), HashMap::new());
    for idx in 0..src.len() {
        let pair = (src[idx], dst[idx]);
        if is_horizontal_or_vertical(pair) {
            for point in points_in_line(pair) {
                let val = match cover_p1.remove(&point) {
                    Some(val) => val + 1,
                    None => 1,
                };
                cover_p1.insert(point, val);
            }
        }

        if is_horizontal_or_vertical(pair) | is_diagonal(pair) {
            for point in points_in_line(pair) {
                let val = match cover_p2.remove(&point) {
                    Some(val) => val + 1,
                    None => 1,
                };
                cover_p2.insert(point, val);
            }
        }
    }

    let res = cover_p1.values().filter(|&z| *z > 1).count();
    println!("Part 1: {}", res);

    let res = cover_p2.values().filter(|&z| *z > 1).count();
    println!("Part 2: {}", res);
}
