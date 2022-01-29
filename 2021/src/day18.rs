use std::fs;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    depth: usize,
}
#[derive(Debug, Clone)]
struct Tree {
    nodes: Vec<Node<i32>>,
}
impl Tree {
    fn new(line: &str) -> Self {
        let (mut nodes, mut depth) = (Vec::new(), 0);
        for char in line.chars() {
            match char {
                '[' => depth += 1,
                ']' => depth -= 1,
                '0'..='9' => nodes.push(Node {
                    value: char.to_digit(10).unwrap() as i32,
                    depth,
                }),
                _ => (),
            }
        }
        Tree { nodes }
    }
    fn add(&mut self, other: &Tree) {
        self.nodes.extend(other.nodes.clone());
        self.nodes.iter_mut().for_each(|z| z.depth += 1);
        self.reduce();
    }
    fn reduce(&mut self) {
        loop {
            while self.explode() {}
            if !self.split() {
                break;
            }
        }
    }
    fn explode(&mut self) -> bool {
        let toexplode = |z: &[Node<i32>]| (z[0].depth > 4) && (z[0].depth == z[1].depth);
        match self.nodes.windows(2).position(toexplode) {
            Some(idx) => {
                let (lf, rg) = (self.nodes[idx].value, self.nodes[idx + 1].value);
                self.nodes[idx].value = 0;
                self.nodes[idx].depth -= 1;
                self.nodes.remove(idx + 1);
                if idx > 0 {
                    self.nodes[idx - 1].value += lf;
                }
                if idx < self.nodes.len() - 1 {
                    self.nodes[idx + 1].value += rg;
                }
                true
            }
            None => false,
        }
    }
    fn split(&mut self) -> bool {
        match self.nodes.iter().position(|z| z.value > 9) {
            Some(idx) => {
                self.nodes.insert(idx + 1, Node { ..self.nodes[idx] });
                self.nodes[idx + 1].value = (self.nodes[idx].value + 1) / 2;
                self.nodes[idx + 1].depth += 1;
                self.nodes[idx].value /= 2;
                self.nodes[idx].depth += 1;
                true
            }
            None => false,
        }
    }
    fn traverse<T: Clone>(&self, conv: impl Fn(i32) -> T, eval: impl Fn(&T, &T) -> T) -> T {
        let mut stack = Vec::new();
        for node in &self.nodes {
            stack.push(Node {
                value: conv(node.value),
                depth: node.depth,
            });
            while stack.len() > 1 {
                if stack[stack.len() - 1].depth != stack[stack.len() - 2].depth {
                    break;
                }
                if let [lf, rg] = &stack.split_off(stack.len() - 2)[..] {
                    stack.push(Node {
                        value: eval(&lf.value, &rg.value),
                        depth: lf.depth - 1,
                    });
                }
            }
        }
        stack[0].value.clone()
    }
    fn magnitude(&self) -> i32 {
        let conv = |value: i32| value;
        let eval = |lf: &i32, rg: &i32| lf * 3 + rg * 2;
        self.traverse(conv, eval)
    }
    #[allow(dead_code)]
    fn print(&self) -> String {
        let conv = |value: i32| format!("{}", value);
        let eval = |lf: &String, rg: &String| format!("[{},{}]", lf, rg);
        self.traverse(conv, eval)
    }
}

pub fn day18() {
    let input = "data/input-18.txt";
    let file: String = fs::read_to_string(input).unwrap();
    let vals: Vec<_> = file.lines().map(Tree::new).collect();

    let mut sum = vals[0].clone();
    if vals.len() > 1 {
        for val in vals.iter().skip(1) {
            sum.add(val);
        }
    }
    let res = sum.magnitude();
    println!("Part 1: {res}");
    assert_eq!(res, 4417);

    let mut max_sum = 0;
    for i in 0..vals.len() {
        for j in 0..vals.len() {
            if i == j {
                continue;
            }
            let mut sum = vals[i].clone();
            sum.add(&vals[j]);
            max_sum = i32::max(max_sum, sum.magnitude());
        }
    }
    println!("Part 2: {max_sum}");
    assert_eq!(max_sum, 4796);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_print() {
        let input = "[[[[[9,8],1],2],3],4]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[7,[6,[5,[4,[3,2]]]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[6,[5,[4,[3,2]]]],1]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[1,[[[9,3],9],[[9,0],[0,7]]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[[5,[7,4]],7],1]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[[[4,2],2],6],[8,7]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[[5,[2,8]],4],[5,[[9,9],0]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[[[5,4],[7,7]],8],[[8,3],8]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[9,3],[[9,9],[6,[4,9]]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[7,[5,[[3,8],[1,4]]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[2,[2,2]],[8,[8,1]]]";
        assert_eq!(Tree::new(input).print(), input);
        let input = "[[[[0,[2,2]],[3,3]],[4,4]],5]";
        assert_eq!(Tree::new(input).print(), input);
    }

    #[test]
    fn test_magnitude() {
        let input = "[[1,2],[[3,4],5]]";
        assert_eq!(Tree::new(input).magnitude(), 143);
        let input = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        assert_eq!(Tree::new(input).magnitude(), 1384);
        let input = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
        assert_eq!(Tree::new(input).magnitude(), 445);
        let input = "[[[[3,0],[5,3]],[4,4]],[5,5]]";
        assert_eq!(Tree::new(input).magnitude(), 791);
        let input = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
        assert_eq!(Tree::new(input).magnitude(), 1137);
        let input = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
        assert_eq!(Tree::new(input).magnitude(), 3488);
    }
}
