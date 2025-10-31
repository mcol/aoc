use std::collections::HashMap;
use std::fs;

pub fn day14() {
    let input = "data/input-14.txt";
    let file = fs::read_to_string(input).unwrap();

    let (mut template, mut read_first_line) = (String::new(), false);
    let (mut fr, mut to, mut vals) = (Vec::new(), Vec::new(), HashMap::new());
    for line in file.lines() {
        let split: Vec<&str> = line.split(" -> ").collect();
        if split.len() == 1 {
            if !read_first_line {
                template = line.to_owned();
                read_first_line = true;
            }
            continue;
        }
        fr.push(split[0]);
        to.push((
            format!("{}{}", &split[0][0..1], split[1]),
            format!("{}{}", split[1], &split[0][1..=1]),
        ));
        vals.insert(split[0].to_owned(), 0i64);
    }

    for i in 0..(template.len() - 1) {
        let sub = &template[i..=(i + 1)];
        vals.insert(sub.to_owned(), vals[sub] + 1);
    }
    for iter in 1..=40 {
        let mut next = HashMap::new();
        for (key, val) in vals {
            let idx = fr.iter().position(|&z| z == key).unwrap();
            let (to0, to1) = &to[idx];
            next.entry(to0.to_owned()).or_insert(0);
            next.entry(to1.to_owned()).or_insert(0);
            next.insert(to0.to_owned(), val + next[to0]);
            next.insert(to1.to_owned(), val + next[to1]);
        }
        vals = next;

        if iter == 10 || iter == 40 {
            let mut letter_count = HashMap::new();
            for (key, val) in vals.clone() {
                let letter = key.chars().next().unwrap();
                letter_count.entry(letter).or_insert(0);
                letter_count.insert(letter, val + letter_count[&letter]);
            }
            let last = template.chars().last().unwrap();
            letter_count.insert(last, letter_count[&last] + 1);

            let (mut min, mut max) = (i64::MAX, 0);
            for (_, val) in letter_count {
                min = i64::min(min, val);
                max = i64::max(max, val);
            }
            println!("Part {}: {}", 1 + (iter == 40) as i8, max - min);
        }
    }
}
