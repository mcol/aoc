use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(vals: (&str, &str)) -> Self {
        Point {
            x: vals.0.to_string().parse::<i32>().unwrap(),
            y: vals.1.to_string().parse::<i32>().unwrap(),
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
    SteppedRange::new(src, dst)
        .map(|(x, y)| Point { x, y })
        .collect()
}

fn is_horizontal_or_vertical(pair: (Point, Point)) -> bool {
    let (src, dst) = pair;
    (src.x == dst.x) | (src.y == dst.y)
}

fn is_diagonal(pair: (Point, Point)) -> bool {
    let (src, dst) = pair;
    src.x.abs_diff(dst.x) == src.y.abs_diff(dst.y)
}

pub fn day05() {
    let input = "data/input-05.txt";
    let file = fs::read_to_string(input).unwrap();
    let (mut src, mut dst) = (Vec::new(), Vec::new());
    for line in file.lines() {
        if let Some((fst, snd)) = line.split_once(" -> ") {
            src.push(Point::new(fst.split_once(',').unwrap()));
            dst.push(Point::new(snd.split_once(',').unwrap()));
        }
    }

    let (mut cover_p1, mut cover_p2) = (HashMap::new(), HashMap::new());
    for idx in 0..src.len() {
        let pair = (src[idx], dst[idx]);
        if is_horizontal_or_vertical(pair) {
            for point in points_in_line(pair) {
                *cover_p1.entry(point).or_insert(0) += 1;
            }
        }

        if is_horizontal_or_vertical(pair) | is_diagonal(pair) {
            for point in points_in_line(pair) {
                *cover_p2.entry(point).or_insert(0) += 1;
            }
        }
    }

    let res = cover_p1.values().into_iter().filter(|&z| *z > 1).count();
    println!("Part 1: {res}");
    assert_eq!(res, 5442);

    let res = cover_p2.values().into_iter().filter(|&z| *z > 1).count();
    println!("Part 2: {res}");
    assert_eq!(res, 19571);
}
