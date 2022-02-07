use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
impl Point {
    fn new(line: &str) -> Self {
        let points: Vec<_> = line.split(',').map(|z| z.parse::<i32>().unwrap()).collect();
        if let [x, y, z] = points[..] {
            Self { x, y, z }
        } else {
            panic!();
        }
    }
    fn euclidean(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as f32).sqrt()
    }
    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
#[derive(Debug, Copy, Clone)]
struct Diff {
    dx: i32,
    dy: i32,
    dz: i32,
    dd: f32,
    i1: usize,
    i2: usize,
}
impl Diff {
    fn new(scanner: &[Point], i1: usize, i2: usize) -> Self {
        let diff = scanner[i1] - scanner[i2];
        Diff {
            dx: diff.x,
            dy: diff.y,
            dz: diff.z,
            dd: diff.euclidean(),
            i1,
            i2,
        }
    }
}
impl PartialEq for Diff {
    fn eq(&self, other: &Self) -> bool {
        if (self.dd - other.dd).abs() > 1e-8 {
            return false;
        }
        let (dx, dy, dz) = (self.dx.abs(), self.dy.abs(), self.dz.abs());
        let (ox, oy, oz) = (other.dx.abs(), other.dy.abs(), other.dz.abs());
        ((dx == ox) && (dy == oy) && (dz == oz))
            || ((dx == oz) && (dy == oy) && (dz == ox))
            || ((dx == oy) && (dy == ox) && (dz == oz))
            || ((dx == ox) && (dy == oz) && (dz == oy))
            || ((dx == oy) && (dy == oz) && (dz == ox))
            || ((dx == oz) && (dy == ox) && (dz == oy))
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Priority {
    idx: usize,
    val: usize,
}
impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}
impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn to_diffs(scanner: &[Point]) -> Vec<Diff> {
    let mut diffs = Vec::new();
    for i in 0..scanner.len() {
        for j in (i + 1)..scanner.len() {
            diffs.push(Diff::new(scanner, i, j));
        }
    }
    diffs
}
fn find_rotation(diffs1: &[Diff], diffs2: &[Diff]) -> u16 {
    let mut rotations = HashMap::new();
    for idx in 0..diffs1.len() {
        match find_rotation_each(&diffs1[idx], &diffs2[idx]) {
            Some(key) => *rotations.entry(key).or_insert(0) += 1,
            None => continue,
        }
    }
    rotations
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(&k, _)| k)
        .unwrap()
}
fn find_rotation_each(d1: &Diff, d2: &Diff) -> Option<u16> {
    if d1 == d2 {
        let (dx, dy, dz) = (d1.dx.abs(), d1.dy.abs(), d1.dz.abs());
        let (ox, oy, oz) = (d2.dx.abs(), d2.dy.abs(), d2.dz.abs());
        if dx == ox {
            match (dy == oy) && (dz == oz) {
                true => return Some(123),
                false => return Some(132),
            }
        }
        if dx == oy {
            match (dy == ox) && (dz == oz) {
                true => return Some(213),
                false => return Some(231),
            }
        }
        if dx == oz {
            match (dy == ox) && (dz == oy) {
                true => return Some(312),
                false => return Some(321),
            }
        }
    }
    None
}
fn apply_rotation(scanner: &[Point], rotation: u16) -> Vec<Point> {
    let mut rotated = Vec::new();
    for val in scanner.iter() {
        let corrpoint = match rotation {
            123 => Point {
                x: val.x,
                y: val.y,
                z: val.z,
            },
            132 => Point {
                x: val.x,
                y: val.z,
                z: val.y,
            },
            213 => Point {
                x: val.y,
                y: val.x,
                z: val.z,
            },
            231 => Point {
                x: val.y,
                y: val.z,
                z: val.x,
            },
            312 => Point {
                x: val.z,
                y: val.x,
                z: val.y,
            },
            321 => Point {
                x: val.z,
                y: val.y,
                z: val.x,
            },
            _ => panic!(),
        };
        rotated.push(corrpoint);
    }
    rotated
}
fn match_points(d1: &[Diff], d2: &[Diff]) -> Vec<(usize, usize)> {
    let mut matches: HashMap<usize, HashSet<_>> = HashMap::new();
    for (idx, &val) in d2.iter().enumerate() {
        let (p1_i1, p1_i2) = (d1[idx].i1, d1[idx].i2);
        for v2 in [val.i1, val.i2] {
            let other = HashSet::from([p1_i1, p1_i2]);
            let v1 = match matches.get(&v2) {
                Some(this) => this.intersection(&other).copied().collect(),
                None => other,
            };
            matches.insert(v2, v1);
        }
    }
    matches
        .iter()
        .map(|(&v2, v1)| (v1.iter().cloned().next().unwrap(), v2))
        .collect()
}
fn fix_orientation(scanner: &mut [Point], cx: bool, cy: bool, cz: bool) {
    for val in scanner.iter_mut() {
        if cx {
            val.x = -val.x;
        }
        if cy {
            val.y = -val.y;
        }
        if cz {
            val.z = -val.z;
        }
    }
}

pub fn day19() {
    let input = "data/input-19.txt";
    let file: String = fs::read_to_string(input).unwrap();

    let (mut scanners, mut vals) = (Vec::new(), Vec::new());
    for line in file.lines().skip(1) {
        if line.contains("---") {
            scanners.push(vals);
            vals = Vec::new();
            continue;
        }
        if line.is_empty() {
            continue;
        }
        vals.push(Point::new(line));
    }
    scanners.push(vals);
    let num_scanners = scanners.len();
    let diffs: Vec<_> = scanners.iter().map(|z| to_diffs(z)).collect();

    let mut s0: HashSet<_> = HashSet::from_iter(scanners[0].clone());
    let (mut scanner_done, mut num_done) = (vec![false; num_scanners], 1);
    let mut scanner_pos = vec![Point { x: 0, y: 0, z: 0 }];
    let mut priority = BinaryHeap::new();
    for idx in 1..num_scanners {
        priority.push(Priority { idx, val: 0 });
    }
    while num_done < num_scanners {
        let s0_vec = Vec::from_iter(s0.clone());
        let s0_diff = to_diffs(&s0_vec);
        let (mut next_priority, mut onedone) = (Vec::new(), false);
        while let Some(prio) = priority.pop() {
            let (idx, val) = (prio.idx, prio.val);
            assert!(!scanner_done[idx]);
            if (val == 0) && onedone && (num_done < num_scanners - 5) {
                priority.push(prio);
                break;
            }
            let (s1, s1_diff) = (&scanners[idx], &diffs[idx]);
            let (mut diffs0, mut diffs1) = (Vec::new(), Vec::new());
            let mut sel_points1 = HashSet::new();
            for &val0 in s0_diff.iter() {
                for &val1 in s1_diff.iter() {
                    if val0 == val1 {
                        diffs0.push(val0);
                        diffs1.push(val1);
                        sel_points1.insert(val1.i1);
                        sel_points1.insert(val1.i2);
                    }
                }
            }

            if sel_points1.len() >= 12 {
                let rotation = find_rotation(&diffs0, &diffs1);
                let mut s1_rot = apply_rotation(s1, rotation);

                // find correspondence in matching points
                let corresp = match_points(&diffs0, &diffs1);
                let ((s0_p1_idx, s1_p1_idx), (s0_p2_idx, s1_p2_idx)) = (corresp[0], corresp[1]);

                // find correction accounting for axis orientation
                let dd0 = Diff::new(&s0_vec, s0_p1_idx, s0_p2_idx);
                let dd1 = Diff::new(&s1_rot, s1_p1_idx, s1_p2_idx);
                fix_orientation(
                    &mut s1_rot,
                    dd0.dx != dd1.dx,
                    dd0.dy != dd1.dy,
                    dd0.dz != dd1.dz,
                );

                // add new points to s0
                let correction = s0_vec[s0_p1_idx] - s1_rot[s1_p1_idx];
                for p1 in &s1_rot {
                    s0.insert(*p1 + correction);
                }
                scanner_pos.push(correction);

                scanner_done[idx] = true;
                num_done += 1;
                onedone = true;
            } else {
                next_priority.push(Priority {
                    idx,
                    val: sel_points1.len(),
                });
            }
        }
        for prio in &next_priority {
            priority.push(*prio);
        }
    }
    println!("Part 1: {}", s0.len());
    assert_eq!(s0.len(), 362);

    let mut dist = 0;
    for val0 in &scanner_pos {
        for val1 in &scanner_pos {
            dist = i32::max(dist, (*val0 - *val1).manhattan());
        }
    }
    println!("Part 2: {}", dist);
    assert_eq!(dist, 12204);
}
