import math, sequtils, strutils

type NodeArray = object
    vec: seq[int]
    value: int
    sum_meta: int

proc eval_node(vec_arg: seq[int]): NodeArray =
    let (num_children, len_metadata) = (vec_arg[0], vec_arg[1])
    var vec = vec_arg[2..<len(vec_arg)]
    var (children, sum_meta) = (newSeq[NodeArray](), 0)
    for _ in 1..num_children:
        let child = eval_node(vec)
        vec = child.vec
        sum_meta += child.sum_meta
        children.add(child)
    let metadata = vec[0..<len_metadata]
    sum_meta += sum(metadata)
    var value = 0
    if num_children > 0:
        for idx in metadata:
            if 0 < idx and idx <= len(children):
                value += children[idx - 1].value
    else:
        value = sum(metadata)
    NodeArray(vec: vec[len_metadata..<len(vec)],
              value: value, sum_meta: sum_meta)

let file = readFile("data/input-08.txt").strip().splitLines()
let vals = file[0].split(" ").map(parseInt)
let root = eval_node(vals)

let res1 = root.sum_meta
echo "Part 1: ", res1
assert(res1 == 41849)

let res2 = root.value
echo "Part 2: ", res2
assert(res2 == 32487)
