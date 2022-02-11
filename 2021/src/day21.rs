use std::collections::HashMap;
use std::fs;

struct Die {
    state: usize,
    rolls: usize,
}
impl Die {
    fn new() -> Self {
        Die { state: 1, rolls: 0 }
    }
    fn roll(&mut self) -> usize {
        let mut score = 0;
        for _ in 0..3 {
            score += self.state;
            self.state = (self.state % 100) + 1
        }
        self.rolls += 3;
        score
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Universe {
    player: usize,
    position: Vec<usize>,
    score: Vec<usize>,
}
impl Universe {
    fn new(position: &[usize]) -> Self {
        Universe {
            player: 0,
            position: position.to_owned(),
            score: vec![0, 0],
        }
    }
    fn play(&mut self, player: usize, value: usize) {
        self.position[player] = (self.position[player] + value - 1) % 10 + 1;
        self.score[player] += self.position[player];
        self.player = (self.player + 1) % 2;
    }
    fn has_reached(&self, winning: usize) -> bool {
        (self.score[0] >= winning) || (self.score[1] >= winning)
    }
}
fn dirac(memo: &mut HashMap<Universe, usize>, uni: &Universe) -> usize {
    if memo.contains_key(uni) {
        return *memo.get(uni).unwrap();
    }
    if uni.has_reached(21) {
        return uni.player;
    }
    let (mut wins, current_player) = (0, uni.player);
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                let mut next = uni.clone();
                next.play(current_player, i + j + k);
                wins += dirac(memo, &next);
            }
        }
    }
    memo.insert(uni.clone(), wins);
    wins
}

pub fn day21() {
    let input = "data/input-21.txt";
    let file: String = fs::read_to_string(input).unwrap();

    let mut position = Vec::new();
    for line in file.lines() {
        let val = line.split(' ').last().unwrap();
        position.push(val.parse::<usize>().unwrap());
    }

    let (mut die, mut universe) = (Die::new(), Universe::new(&position));
    while !universe.has_reached(1000) {
        universe.play(universe.player, die.roll());
    }
    let res = universe.score.iter().min().unwrap() * die.rolls;
    println!("Part 1: {}", res);
    assert_eq!(res, 711480);

    let mut memo = HashMap::new();
    let wins = dirac(&mut memo, &Universe::new(&position));
    println!("Part 2: {}", wins);
    assert_eq!(wins, 265845890886828);
}
