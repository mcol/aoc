use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Instruction {
    on: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}
impl Instruction {
    fn new(line: &str) -> Self {
        let (on, rest) = line.split_once(' ').unwrap();
        let points: Vec<_> = rest
            .split(',')
            .map(|z| {
                z[2..]
                    .split_once("..")
                    .map(|(v1, v2)| (v1.parse::<i32>().unwrap(), v2.parse::<i32>().unwrap()))
                    .unwrap()
            })
            .collect();
        Self {
            on: match on {
                "on" => true,
                "off" => false,
                _ => panic!(),
            },
            x: points[0],
            y: points[1],
            z: points[2],
        }
    }
    fn intersect(&self, ins: &Instruction) -> Option<Instruction> {
        let x = (i32::max(self.x.0, ins.x.0), i32::min(self.x.1, ins.x.1));
        let y = (i32::max(self.y.0, ins.y.0), i32::min(self.y.1, ins.y.1));
        let z = (i32::max(self.z.0, ins.z.0), i32::min(self.z.1, ins.z.1));
        if x.0 > x.1 || y.0 > y.1 || z.0 > z.1 {
            return None;
        }
        Some(Self {
            on: self.on,
            x,
            y,
            z,
        })
    }
    fn volume(&self) -> i64 {
        (self.x.1 - self.x.0 + 1) as i64
            * (self.y.1 - self.y.0 + 1) as i64
            * (self.z.1 - self.z.0 + 1) as i64
    }
}

fn build_cube(instructions: &[Instruction]) -> HashMap<Instruction, i16> {
    let mut cube = HashMap::new();
    for instr in instructions {
        let to_add: Vec<_> = cube
            .iter()
            .filter_map(|(other_instr, &count): (&Instruction, &i16)| {
                instr.intersect(other_instr).map(|int| (int, -count))
            })
            .collect();
        for (key, val) in to_add {
            *cube.entry(key).or_insert(0) += val;
        }
        if instr.on {
            *cube.entry(*instr).or_insert(0) += 1;
        }
    }
    cube
}

pub fn day22() {
    let input = "data/input-22.txt";
    let file: String = fs::read_to_string(input).unwrap();
    let instructions: Vec<_> = file.lines().map(Instruction::new).collect();

    let bounded_instructions: Vec<_> = instructions
        .iter()
        .filter_map(|&z| {
            z.intersect(&Instruction {
                on: true,
                x: (-50, 50),
                y: (-50, 50),
                z: (-50, 50),
            })
        })
        .collect();
    let res = build_cube(&bounded_instructions)
        .iter()
        .fold(0, |acc, (cube, &count)| acc + cube.volume() * count as i64);
    println!("Part 1: {}", res);
    assert_eq!(res, 570915);

    let res = build_cube(&instructions)
        .iter()
        .fold(0, |acc, (cube, &count)| acc + cube.volume() * count as i64);
    println!("Part 2: {}", res);
    assert_eq!(res, 1268313839428137);
}
