use std::fs;

fn react(polymer: &str, skip: char) -> String {
    let (mut reacted, mut last) = (Vec::new(), ' ');
    for char in polymer.chars() {
        if char.eq_ignore_ascii_case(&skip) {
            continue;
        }
        if char.eq_ignore_ascii_case(&last) && char != last {
            reacted.pop();
            last = match reacted.last() {
                Some(&c) => c,
                None => ' ',
            };
            continue;
        }
        reacted.push(char);
        last = char;
    }
    reacted.into_iter().collect::<String>()
}

pub fn day05() {
    let input = "data/input-05.txt";
    let polymer = fs::read_to_string(input).unwrap().replace('\n', "");

    let res = react(&polymer, ' ').len();
    println!("Part 1: {res}");
    assert_eq!(res, 9296);

    let res = ('a'..='z').fold(polymer.len(), |min, c| {
        usize::min(min, react(&polymer, c).len())
    });
    println!("Part 2: {res}");
    assert_eq!(res, 5534);
}
