use std::fs;

#[derive(Clone)]
struct Marble {
    prev: usize,
    next: usize,
}
struct Ring {
    curr: usize,
    vals: Vec<Marble>,
}
impl Ring {
    fn new(max_size: usize) -> Self {
        let vals = vec![Marble { prev: 0, next: 0 }; max_size + 1];
        Ring { curr: 0, vals }
    }
    fn add(&mut self, value: usize) {
        let next_value = self.vals[self.curr].next;
        self.vals[value] = Marble {
            prev: self.curr,
            next: next_value,
        };
        self.vals[self.curr].next = value;
        self.vals[next_value].prev = value;
    }
    fn del(&mut self) -> usize {
        let (curr, rem) = (self.curr, &self.vals[self.curr]);
        let (prev_value, next_value) = (rem.prev, rem.next);
        self.vals[prev_value].next = next_value;
        self.vals[next_value].prev = prev_value;
        self.curr = prev_value;
        curr
    }
    fn step(&mut self, steps: usize) {
        for _ in 0..steps {
            self.curr = self.vals[self.curr].next;
        }
    }
    fn back(&mut self, steps: usize) {
        for _ in 0..steps {
            self.curr = self.vals[self.curr].prev;
        }
    }
}

fn compute_scores(num_players: usize, num_marbles: usize) -> Vec<usize> {
    let (mut player, mut scores) = (0, vec![0; num_players]);
    let mut marbles = Ring::new(num_marbles);
    for marble in 1..=num_marbles {
        player = (player + 1) % num_players;
        marbles.step(2);
        match marble % 23 {
            0 => {
                marbles.back(8);
                scores[player] += marble + marbles.del()
            }
            _ => marbles.add(marble),
        }
    }
    scores
}

pub fn day09() {
    let input = "data/input-09.txt";
    let line: Vec<_> = fs::read_to_string(input)
        .unwrap()
        .split(' ')
        .filter_map(|z| z.parse::<usize>().ok())
        .collect();
    let (num_players, num_marbles) = (line[0], line[1]);

    let scores = compute_scores(num_players, num_marbles);
    let &res = scores.iter().max().unwrap();
    println!("Part 1: {res}");
    assert_eq!(res, 385820);

    let scores = compute_scores(num_players, num_marbles * 100);
    let &res = scores.iter().max().unwrap();
    println!("Part 2: {res}");
    assert_eq!(res, 3156297594);
}
