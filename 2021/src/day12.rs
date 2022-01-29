use std::fs;

fn can_be_child(path: &[&str], name: &str, allow_rep: bool) -> bool {
    name == name.to_uppercase() || !path.contains(&name) || allow_rep
}
fn check_rep(path: &[&str]) -> bool {
    let (mut unique, mut found_rep) = (Vec::new(), false);
    for node in path
        .iter()
        .filter(|&&z| z != z.to_uppercase() && z != "start")
    {
        if !unique.contains(node) {
            unique.push(node);
        } else if !found_rep {
            found_rep = true;
        } else {
            return false;
        }
    }
    !found_rep
}

pub fn day12() {
    let input = "data/input-12.txt";
    let file = fs::read_to_string(input).unwrap();
    let (mut src, mut dst) = (Vec::new(), Vec::new());
    for line in file.lines() {
        if let Some((fst, snd)) = line.split_once('-') {
            if fst != "end" && snd != "start" {
                src.push(fst);
                dst.push(snd);
            }
            if fst != "start" && snd != "end" {
                src.push(snd);
                dst.push(fst);
            }
        }
    }

    let (mut stack, mut count) = (vec![("start", 0)], 0);
    let mut path = Vec::new();
    while let Some((curr, level)) = stack.pop() {
        path.resize(level, "");
        let mut has_children = false;
        for (idx, _) in src.iter().enumerate().filter(|&z| *z.1 == curr) {
            let dest = dst[idx];
            if can_be_child(&path, dest, false) {
                stack.push((dest, path.len() + 1));
                has_children = true;
            }
        }
        if has_children {
            path.push(curr);
        } else if curr == "end" {
            count += 1;
            path.push(curr);
        }
    }
    println!("Part 1: {count}");
    assert_eq!(count, 4691);

    let (mut stack, mut count) = (vec![("start", 0)], 0);
    let mut path = Vec::new();
    while let Some((curr, level)) = stack.pop() {
        path.resize(level, "");
        let allow_rep = check_rep(&path);
        let mut has_children = false;
        let curr_addable = can_be_child(&path, curr, allow_rep);
        if curr_addable {
            for (idx, _) in src.iter().enumerate().filter(|&z| *z.1 == curr) {
                let dest = dst[idx];
                if can_be_child(&path, dest, allow_rep) {
                    stack.push((dest, path.len() + 1));
                    has_children = true;
                }
            }
        }
        if has_children {
            if curr_addable {
                path.push(curr);
            }
        } else if curr == "end" {
            count += 1;
            path.push(curr);
        }
    }
    println!("Part 2: {count}");
    assert_eq!(count, 140718);
}
