use std::fmt;
use std::fs;

#[derive(Debug)]
enum Op {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}
impl Op {
    fn new(op: &str) -> Self {
        match op {
            "inp" => Op::Inp,
            "add" => Op::Add,
            "mul" => Op::Mul,
            "div" => Op::Div,
            "mod" => Op::Mod,
            "eql" => Op::Eql,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Memory {
    W,
    X,
    Y,
    Z,
}
impl Memory {
    fn new(string: &str) -> Self {
        match string {
            "w" => Memory::W,
            "x" => Memory::X,
            "y" => Memory::Y,
            "z" => Memory::Z,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Term {
    Mem(Memory),
    Val(i64),
}

#[derive(Debug)]
struct State {
    memory: [i64; 4],
}
impl State {
    fn new() -> Self {
        State { memory: [0; 4] }
    }
    fn set(&mut self, val: &Term, value: i64) {
        if let &Term::Mem(mem) = val {
            self.memory[mem as usize] = value
        }
    }
    fn get(&self, val: &Term) -> i64 {
        match *val {
            Term::Mem(mem) => self.memory[mem as usize],
            Term::Val(num) => num,
        }
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "w: {}, x: {}, y: {}, z: {}",
            self.memory[0], self.memory[1], self.memory[2], self.memory[3]
        )
    }
}

struct Instruction {
    op: Op,
    v1: Term,
    v2: Term,
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op = match self.op {
            Op::Inp => "->",
            Op::Add => "+=",
            Op::Mul => "*=",
            Op::Div => "/=",
            Op::Mod => "%=",
            Op::Eql => "==",
        };
        match self.op {
            Op::Inp => write!(f, " {} {:?}", op, self.v1),
            _ => write!(f, " {:?} {} {:?}", self.v1, op, self.v2),
        }
    }
}

#[derive(Debug)]
enum ProgramError {
    InvalidInputLength,
    InvalidModelNumber,
}

struct Program {
    instructions: Vec<Instruction>,
}
impl Program {
    fn new(file: &str) -> Self {
        Program {
            instructions: file.lines().map(Program::parse_line).collect(),
        }
    }
    fn parse_line(line: &str) -> Instruction {
        let line: Vec<_> = line.split(' ').collect();
        Instruction {
            op: Op::new(line[0]),
            v1: Term::Mem(Memory::new(line[1])),
            v2: match line.len() {
                2 => Term::Val(0),
                3 => match line[2].parse::<i64>() {
                    Ok(value) => Term::Val(value),
                    Err(_) => Term::Mem(Memory::new(line[2])),
                },
                _ => panic!(),
            },
        }
    }
    fn run(&self, input: i64, trace: bool) -> Result<State, ProgramError> {
        let input: Vec<_> = input
            .to_string()
            .chars()
            .filter_map(|z| z.to_digit(10))
            .collect();
        if input.len() != 14 {
            eprintln!("The input must contain 14 digits.");
            return Err(ProgramError::InvalidInputLength);
        }
        let (mut state, mut idx_input) = (State::new(), 0);
        for instr in &self.instructions {
            let (v1, v2) = (state.get(&instr.v1), state.get(&instr.v2));
            state.set(
                &instr.v1,
                match instr.op {
                    Op::Inp => {
                        let val = input[idx_input] as i64;
                        idx_input += 1;
                        val
                    }
                    Op::Add => v1 + v2,
                    Op::Mul => v1 * v2,
                    Op::Div => v1 / v2,
                    Op::Mod => v1 % v2,
                    Op::Eql => (v1 == v2) as i64,
                },
            );
            if trace {
                println!("{instr}:\t{state}");
            }
        }
        match state.memory[Memory::Z as usize] {
            0 => Ok(state),
            _ => Err(ProgramError::InvalidModelNumber),
        }
    }
}

pub fn day24() {
    let input = "data/input-24.txt";
    let file: String = fs::read_to_string(input).unwrap();
    let program = Program::new(&file);

    let max_input = 49917929934999;
    match program.run(max_input, false) {
        Ok(_) => {
            let res = max_input;
            println!("Part 1: {res}");
            assert_eq!(res, 49917929934999);
        }
        Err(err) => panic!("{err:?}"),
    }

    let min_input = 11911316711816;
    match program.run(min_input, false) {
        Ok(_) => {
            let res = min_input;
            println!("Part 2: {res}");
            assert_eq!(res, 11911316711816);
        }
        Err(err) => panic!("{err:?}"),
    }
}
