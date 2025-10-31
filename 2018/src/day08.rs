use std::fs;

struct NodeSummary<'a> {
    vec: &'a [u16],
    value: u16,
    sum_meta: u16,
}

fn eval_node(vec: &[u16]) -> NodeSummary<'_> {
    let (num_children, len_metadata) = (vec[0], vec[1]);
    let (mut children, mut sum_meta) = (Vec::new(), 0);
    let mut vec = vec.split_at(2).1;
    for _ in 0..num_children {
        let child = eval_node(vec);
        vec = child.vec;
        sum_meta += child.sum_meta;
        children.push(child);
    }
    let (metadata, vec) = vec.split_at(len_metadata as usize);
    let value = match num_children {
        0 => metadata.iter().cloned().sum(),
        _ => metadata.iter().fold(0, |acc, &z| {
            acc + match z <= num_children {
                true => children[z as usize - 1].value,
                false => 0,
            }
        }),
    };
    sum_meta += metadata.iter().cloned().sum::<u16>();
    NodeSummary {
        vec,
        value,
        sum_meta,
    }
}

pub fn day08() {
    let input = "data/input-08.txt";
    let license: Vec<_> = fs::read_to_string(input)
        .unwrap()
        .replace('\n', "")
        .split(' ')
        .map(|z| z.parse::<u16>().unwrap())
        .collect();
    let root = eval_node(&license);

    let res = root.sum_meta;
    println!("Part 1: {res}");
    assert_eq!(res, 41849);

    let res = root.value;
    println!("Part 2: {res}");
    assert_eq!(res, 32487);
}
