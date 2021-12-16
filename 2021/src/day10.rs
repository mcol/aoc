use std::fs;

fn check_match(x: &char, y: &char) -> i32 {
    match y {
        ')' if *x != '(' => 3,
        ']' if *x != '[' => 57,
        '}' if *x != '{' => 1197,
        '>' if *x != '<' => 25137,
        _ => 0,
    }
}

pub fn day10() {
    let input = "data/input-10.txt";
    let file = fs::read_to_string(input).unwrap();
    let signs: Vec<_> = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let (mut scores_p1, mut scores_p2) = (Vec::new(), Vec::new());
    'next_line: for line in &signs {
        let mut stack = Vec::new();
        for sign in line {
            match sign {
                '(' | '[' | '{' | '<' => stack.push(sign),
                _ => {
                    let res = check_match(stack.pop().unwrap(), sign);
                    if res > 0 {
                        scores_p1.push(res);
                        continue 'next_line;
                    }
                }
            }
        }

        let val: i64 = stack.iter().rev().fold(0, |acc, z| {
            acc * 5
                + match z {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                }
        });
        scores_p2.push(val);
    }

    println!("Part 1: {}", scores_p1.iter().sum::<i32>());

    scores_p2.sort_unstable();
    println!("Part 2: {:?}", scores_p2[scores_p2.len() / 2]);
}
