use std::fs;

struct Card {
    vals: Vec<i32>,
    horz: Vec<i32>,
    vert: Vec<i32>,
    left: i32,
}

impl Card {
    fn new(vals: Vec<i32>) -> Self {
        Card {
            left: vals.iter().sum(),
            horz: vec![0; 5],
            vert: vec![0; 5],
            vals: vals,
        }
    }
    fn check(&mut self, val: i32) -> bool {
        match self.vals.iter().position(|&z| z == val) {
            Some(idx) => {
                self.left -= val;
                let (row, col) = (idx / 5, idx % 5);
                self.horz[row] += 1;
                self.vert[col] += 1;
                (self.horz[row] == 5) | (self.vert[col] == 5)
            }
            None => false,
        }
    }
}

pub fn day04() {
    let input = "data/input-04.txt";
    let file = fs::read_to_string(input).unwrap();

    let (mut drawn, mut cards) = (Vec::new(), Vec::new());
    let mut vals = Vec::new();
    let mut read_first_line = false;

    for line in file.lines() {
        if !read_first_line {
            for token in line.split(",") {
                drawn.push(token.to_string().parse::<i32>().expect(""));
            }
            read_first_line = true;
            continue;
        }
        if line.len() == 0 {
            continue;
        }
        for token in line.split(" ") {
            if token.len() == 0 {
                continue;
            }
            vals.push(token.to_string().parse::<i32>().expect(""));
        }
        if vals.len() == 25 {
            cards.push(Card::new(vals));
            vals = Vec::new();
        }
    }

    let num_cards = cards.len();
    let mut valid_card = vec![1; num_cards];
    let mut solved_part1 = false;
    'outer: for curr in drawn {
        let mut idx = 0;
        for card in &mut cards {
            if (valid_card[idx] == 1) & card.check(curr) {
                if !solved_part1 {
                    println!("Part 1: {}", curr * card.left);
                    solved_part1 = true;
                }
                valid_card[idx] = 0;
                if valid_card.iter().sum::<i32>() == 0 {
                    println!("Part 2: {}", curr * card.left);
                    break 'outer;
                }
            }
            idx += 1;
        }
    }
}
