use std::fs;

#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    value: usize,
    children: usize,
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}
fn to_usize(code: &str) -> usize {
    usize::from_str_radix(code, 2).unwrap()
}
fn parse(code: &str) -> Vec<Packet> {
    let mut packets = Vec::new();
    parse_code(&mut packets, code);
    packets
}
fn parse_code(packets: &mut Vec<Packet>, code: &str) -> usize {
    let (head, body) = code.split_at(6);
    let version_type = (to_usize(&head[..3]), to_usize(&head[3..]));
    6 + match version_type.1 {
        4 => parse_literal(packets, version_type, body),
        _ => parse_operator(packets, version_type, body),
    }
}
fn parse_literal(packets: &mut Vec<Packet>, vt: (usize, usize), code: &str) -> usize {
    let (mut number, mut start, block_size) = (String::new(), 0, 5);
    loop {
        number.push_str(&code[(start + 1)..(start + block_size)]);
        match &code[start..(start + 1)] {
            "1" => start += block_size,
            _ => break,
        }
    }
    packets.push(Packet {
        version: vt.0,
        type_id: vt.1,
        value: usize::from_str_radix(number.as_str(), 2).unwrap(),
        children: 0,
    });
    start + block_size
}
fn parse_operator(packets: &mut Vec<Packet>, vt: (usize, usize), code: &str) -> usize {
    let mut children = 0;
    let read = match &code[..1] {
        "0" => {
            let (mut from, last) = (16, to_usize(&code[1..16]) + 16);
            while from < last {
                from += parse_code(packets, &code[from..]);
                children += 1;
            }
            last
        }
        "1" => {
            let mut from = 12;
            children = to_usize(&code[1..12]);
            for _ in 0..children {
                from += parse_code(packets, &code[from..]);
            }
            from
        }
        _ => unreachable!(),
    };
    packets.push(Packet {
        version: vt.0,
        type_id: vt.1,
        value: 0,
        children,
    });
    read
}
fn evaluate(packets: &[Packet]) -> usize {
    let mut stack = Vec::new();
    for packet in packets {
        let vals = stack.iter().rev().take(packet.children);
        let res = match packet.type_id {
            0 => vals.sum(),
            1 => vals.product(),
            2 => *vals.min().unwrap(),
            3 => *vals.max().unwrap(),
            4 => packet.value,
            5 => {
                let vals: Vec<_> = vals.collect();
                (vals[1] > vals[0]) as usize
            }
            6 => {
                let vals: Vec<_> = vals.collect();
                (vals[1] < vals[0]) as usize
            }
            7 => {
                let vals: Vec<_> = vals.collect();
                (vals[1] == vals[0]) as usize
            }
            _ => panic!("Unexpected type_id: {}", packet.type_id),
        };
        stack.truncate(stack.len() - packet.children);
        stack.push(res);
    }
    stack.pop().unwrap()
}

pub fn day16() {
    let input = "data/input-16.txt";
    let hex: String = fs::read_to_string(input).unwrap().replace('\n', "");
    let bin: String = hex.chars().map(to_binary).collect();

    let packets = parse(&bin[..]);
    let res = packets.iter().map(|z| z.version).sum::<usize>();
    println!("Part 1: {res}");
    assert_eq!(res, 996);

    let res = evaluate(&packets);
    println!("Part 2: {res}");
    assert_eq!(res, 96257984154);
}
