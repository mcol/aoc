use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

#[derive(Eq, PartialEq)]
struct Node {
    node: (usize, usize),
    dist: u32,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbours(nrows: usize, ncols: usize, node: (usize, usize)) -> Vec<(usize, usize)> {
    let (mut v, (i, j)) = (Vec::new(), node);
    if i > 0 {
        v.push((i - 1, j));
    }
    if i < nrows - 1 {
        v.push((i + 1, j));
    }
    if j > 0 {
        v.push((i, j - 1));
    }
    if j < ncols - 1 {
        v.push((i, j + 1));
    }
    v
}

fn dijkstra(vals: &[Vec<u32>]) -> u32 {
    let (nrows, ncols) = (vals.len(), vals[0].len());
    let mut queued = BinaryHeap::from([Node {
        node: (0, 0),
        dist: 0,
    }]);
    let mut dists = vec![vec![u32::MAX; ncols]; nrows];
    dists[0][0] = 0;

    while let Some(Node { node, dist }) = queued.pop() {
        for (i_, j_) in neighbours(nrows, ncols, node) {
            if dists[i_][j_] > dist + vals[i_][j_] {
                dists[i_][j_] = dist + vals[i_][j_];
                queued.push(Node {
                    node: (i_, j_),
                    dist: dists[i_][j_],
                });
            }
        }
    }
    dists[nrows - 1][ncols - 1]
}

pub fn day15() {
    let input = "data/input-15.txt";
    let file = fs::read_to_string(input).unwrap();
    let vals: Vec<_> = file
        .lines()
        .map(|z| z.chars().filter_map(|zz| zz.to_digit(10)).collect())
        .collect();

    let res = dijkstra(&vals);
    println!("Part 1: {res}");
    assert_eq!(res, 769);

    let (nrows, ncols) = (vals.len(), vals[0].len());
    let mut vals_5 = vec![vec![0; ncols * 5]; nrows * 5];
    for i in 0..nrows {
        for j in 0..ncols {
            for k_r in 0..5 {
                for k_c in 0..5 {
                    let mut newval = vals[i][j] + (k_r + k_c) as u32;
                    if newval > 9 {
                        newval -= 9;
                    }
                    vals_5[i + nrows * k_r][j + ncols * k_c] = newval;
                }
            }
        }
    }
    let res = dijkstra(&vals_5);
    println!("Part 2: {res}");
    assert_eq!(res, 2963);
}
