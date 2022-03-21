use std::fs;

struct Image {
    row: Vec<isize>,
    col: Vec<isize>,
    hor_vel: Vec<isize>,
    ver_vel: Vec<isize>,
}
impl Image {
    fn new() -> Self {
        Image {
            row: Vec::new(),
            col: Vec::new(),
            hor_vel: Vec::new(),
            ver_vel: Vec::new(),
        }
    }
    fn add_point(&mut self, point: &[isize]) {
        self.row.push(point[0]);
        self.col.push(point[1]);
        self.hor_vel.push(point[2]);
        self.ver_vel.push(point[3]);
    }
    fn move_point(&mut self, moves: isize) {
        for idx in 0..self.row.len() {
            self.row[idx] += self.hor_vel[idx] * moves;
            self.col[idx] += self.ver_vel[idx] * moves;
        }
    }
    fn sizes(&self) -> (usize, usize, isize, isize) {
        let &xmin = self.row.iter().min().unwrap();
        let &ymin = self.col.iter().min().unwrap();
        let wid = self.row.iter().max().unwrap() - xmin;
        let hei = self.col.iter().max().unwrap() - ymin;
        (wid as usize, hei as usize, xmin, ymin)
    }
    fn area(&self) -> usize {
        let (wid, hei, _, _) = self.sizes();
        wid * hei
    }
    fn draw(&self) {
        let (wid, hei, xmin, ymin) = self.sizes();
        let mut render = vec![vec![' '; wid + 1]; hei + 1];
        for (i, j) in self.row.iter().zip(self.col.iter()) {
            render[(j - ymin) as usize][(i - xmin) as usize] = '#';
        }
        for row in render {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

pub fn day10() {
    let input = "data/input-10.txt";
    let file = fs::read_to_string(input)
        .unwrap()
        .replace("position=<", "")
        .replace("> velocity=<", ",")
        .replace('>', "");
    let mut image = Image::new();
    for line in file.lines() {
        let line: Vec<_> = line
            .split(',')
            .map(|z| z.trim().parse::<isize>().unwrap())
            .collect();
        image.add_point(&line);
    }

    let (mut min_area, mut min_iter) = (image.area(), 0);
    for iter in 1..11000 {
        image.move_point(1);
        let area = image.area();
        if area < min_area {
            (min_area, min_iter) = (area, iter);
        } else {
            image.move_point(-1);
            break;
        }
    }

    println!("Part 1:");
    image.draw();

    let res = min_iter;
    println!("Part 2: {res}");
    assert_eq!(res, 10888);
}
