use std::fs;

enum Type {
    A,
    B,
    C,
    D,
}

fn energy(typ: Type) -> i32 {
    match typ {
        Type::A => 1,
        Type::B => 10,
        Type::C => 100,
        Type::D => 1000,
    }
}

pub fn day23() {
    let _input = "data/input-23.txt";
    let input = "data/example-23.txt";
    let file: String = fs::read_to_string(input).unwrap();

    let mut instructions = Vec::new();
    for line in file.lines() {
        instructions.push(line);
    }

    println!("Part 1: {}", 0);
}
