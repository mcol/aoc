use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;

type XY = (i32, i32);
struct SparseImage {
    coords: HashSet<XY>,
    head: XY,
    tail: XY,
}
impl SparseImage {
    fn new() -> Self {
        Self {
            coords: HashSet::new(),
            head: (0, 0),
            tail: (0, 0),
        }
    }
    fn insert(&mut self, x: i32, y: i32) -> bool {
        self.head = (i32::min(self.head.0, x), i32::min(self.head.1, y));
        self.tail = (i32::max(self.tail.0, x), i32::max(self.tail.1, y));
        self.coords.insert((x, y))
    }
    fn xcoords(&self) -> RangeInclusive<i32> {
        (self.head.0 - 2)..=(self.tail.0 + 2)
    }
    fn ycoords(&self) -> RangeInclusive<i32> {
        (self.head.1 - 2)..=(self.tail.1 + 2)
    }
    fn len(&self) -> usize {
        self.coords.len()
    }
    fn to_decimal(&self, x: i32, y: i32, outside: bool) -> usize {
        let (mut sum, mut pow) = (0, 256);
        for j in [-1, 0, 1] {
            for i in [-1, 0, 1] {
                let (xx, yy) = (x + i, y + j);
                let val = match (xx >= self.head.0)
                    && (xx <= self.tail.0)
                    && (yy >= self.head.1)
                    && (yy <= self.tail.1)
                {
                    true => self.coords.contains(&(xx, yy)),
                    false => outside,
                };
                sum += val as usize * pow;
                pow /= 2;
            }
        }
        sum
    }
}

pub fn day20() {
    let input = "data/input-20.txt";
    let file: String = fs::read_to_string(input).unwrap();

    let (mut enhance, mut read_first_line, mut y) = (Vec::new(), false, 0);
    let mut image = SparseImage::new();
    for line in file.lines() {
        if !read_first_line {
            enhance = line
                .chars()
                .map(|z| match z {
                    '.' => false,
                    '#' => true,
                    _ => panic!(),
                })
                .collect();
            read_first_line = true;
            continue;
        }
        if line.is_empty() {
            continue;
        }
        for (x, _) in line.chars().enumerate().filter(|(_, z)| *z == '#') {
            image.insert(x as i32, y);
        }
        y += 1;
    }

    let mut outside = false;
    for iter in 1..=50 {
        let mut enhanced = SparseImage::new();
        for x in image.xcoords() {
            for y in image.ycoords() {
                if enhance[image.to_decimal(x, y, outside)] {
                    enhanced.insert(x, y);
                }
            }
        }
        image = enhanced;
        outside = enhance[outside as usize * 511];
        if iter == 2 {
            println!("Part 1: {}", image.len());
            assert_eq!(image.len(), 5065);
        }
    }
    println!("Part 2: {}", image.len());
    assert_eq!(image.len(), 14790);
}
