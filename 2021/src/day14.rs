use std::collections::HashMap;
use std::fs;

pub fn day14() {
    let input = "data/input-14.txt";
    let file = fs::read_to_string(input).unwrap();
    let (mut template, mut read_first_line) = (String::new(), false);
    let (mut fr, mut to, mut vals) = (Vec::new(), Vec::new(), HashMap::new());
    for line in file.lines() {
        if let Some((fst, snd)) = line.split_once(" -> ") {
            fr.push(fst);
            to.push((
                format!("{}{}", &fst[0..1], snd),
                format!("{}{}", snd, &fst[1..=1]),
            ));
            vals.insert(fst.to_owned(), 0i64);
        } else if !read_first_line {
            template = line.to_owned();
            read_first_line = true;
        }
    }

    for i in 0..(template.len() - 1) {
        let sub = &template[i..=(i + 1)];
        vals.insert(sub.to_owned(), vals[sub] + 1);
    }
    for iter in 1..=40 {
        let mut next = HashMap::new();
        for (key, val) in vals {
            let idx = fr.iter().position(|&z| z == key).unwrap();
            let (to0, to1) = to[idx].to_owned();
            *next.entry(to0).or_insert(0) += val;
            *next.entry(to1).or_insert(0) += val;
        }
        vals = next;

        if iter == 10 || iter == 40 {
            let mut letter_count = HashMap::new();
            for (key, val) in &vals {
                let letter = key.chars().next().unwrap();
                *letter_count.entry(letter).or_insert(0) += val;
            }
            let last = template.chars().last().unwrap();
            letter_count.insert(last, letter_count[&last] + 1);

            let (min, max) = letter_count.iter().fold((i64::MAX, 0), |acc, (_, &val)| {
                (i64::min(acc.0, val), i64::max(acc.1, val))
            });
            println!("Part {}: {}", 1 + (iter == 40) as i8, max - min);
            match iter {
                10 => assert_eq!(max - min, 3009),
                40 => assert_eq!(max - min, 3459822539451),
                _ => panic!(),
            }
        }
    }
}
